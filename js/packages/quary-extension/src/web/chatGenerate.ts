import * as vscode from 'vscode'
import {
  collectResults,
  Err,
  ErrorCodes,
  isErr,
  Ok,
  Result,
} from '@shared/result'
import { z } from 'zod'
import { getServices, preInitSetup } from './services'
import { chatReturnFinalChat } from './chatHelpers'

/**
 * chatGenerate gets called when a user wants to generate a model in the chat model by using the `/generate` command.
 *
 * If references to sql files are provided in the chat request, the function goes straight to the LLM and returns the final chat.
 * If no references to sql files are provide in the chat request, the function does a first round trip to the LLM to get the references that may be relevant.
 */
export const chatGenerate = async (
  extensionContext: vscode.ExtensionContext,
  model: vscode.LanguageModelChat,
  request: vscode.ChatRequest,
  _: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
): Promise<Result<void>> => {
  const modelNames = await getModels(extensionContext, model, request, token)
  if (isErr(modelNames)) {
    throw modelNames.error
  }

  const services = await getServices(extensionContext)
  const details = await preInitSetup(services)
  if (isErr(details)) {
    return details
  }
  const prompt = await services.rust.returnGenerateModelPrompt({
    projectRoot: details.value.projectRoot,
    prompt: request.prompt,
    references: modelNames.value,
  })
  return chatReturnFinalChat(prompt, model, token, stream)
}

const getModels = async (
  extensionContext: vscode.ExtensionContext,
  model: vscode.LanguageModelChat,
  request: vscode.ChatRequest,
  token: vscode.CancellationToken,
): Promise<Result<Array<string>>> => {
  if (request.references.length !== 0) {
    const models: Array<Result<string>> = request.references.map(
      (reference) => {
        if (reference.id !== 'vscode.file') {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Please provide only models as files.',
          })
        }
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        const file: string = reference.value.path as string
        if (!file.endsWith('.sql')) {
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: 'Please provide only models as .sql files.',
          })
        }
        const model = file.split('/')[file.split('/').length - 1].split('.')[0]
        return Ok(model)
      },
    )
    const modelNames = collectResults(models)
    if (isErr(modelNames)) {
      throw modelNames.error
    }
    return modelNames
  }
  const services = await getServices(extensionContext)
  const details = await preInitSetup(services)
  if (isErr(details)) {
    return details
  }
  const prompt = await services.rust.returnGenerateModelPromptToSearchForModels(
    {
      projectRoot: details.value.projectRoot,
      prompt: request.prompt,
    },
  )
  if (isErr(prompt)) {
    return prompt
  }
  const promptPrep = [
    vscode.LanguageModelChatMessage.Assistant(prompt.value.agentPrompt),
    vscode.LanguageModelChatMessage.User(prompt.value.userPrompt),
  ]
  // Send the prompt to the AI
  const response = await model.sendRequest(promptPrep, {}, token)
  let fullResponse = ''

  try {
    // consume stream
    for await (const chunk of response.text) {
      fullResponse += chunk
    }
  } catch (e) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message:
        'Could not read the AI response for models: ' + JSON.stringify(e),
    })
  }

  // parse the response that looks like markdown with a json block inside of it and just return the string inside the block
  const jsonBlock = fullResponse.match(/```json\n(.*)\n```/s)
  if (!jsonBlock) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: 'Could not parse the AI response for models.',
    })
  }
  const array = JSON.parse(jsonBlock[1])
  // Parse the AI response
  const returned = zodArray.safeParse(array)
  if (!returned.success) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: 'Could not parse the AI response for models.',
    })
  }
  return Ok(returned.data)
}

const zodArray = z.array(z.string())
