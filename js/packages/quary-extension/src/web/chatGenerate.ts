import * as vscode from 'vscode'
import {
  collectResults,
  Err,
  ErrorCodes,
  isErr,
  Ok,
  Result,
} from '@shared/result'
import { getServices, preInitSetup } from './services'

export const chatGenerate = async (
  extensionContext: vscode.ExtensionContext,
  model: vscode.LanguageModelChat,
  request: vscode.ChatRequest,
  _: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
): Promise<Result<void>> => {
  const models: Array<Result<string>> = request.references.map((reference) => {
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
  })
  const modelNames = collectResults(models)
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
  if (isErr(prompt)) {
    return prompt
  }
  const craftedPrompt = [
    vscode.LanguageModelChatMessage.Assistant(prompt.value.agentPrompt),
    vscode.LanguageModelChatMessage.User(prompt.value.userPrompt),
  ]
  const modelRequest = await model.sendRequest(craftedPrompt, {}, token)
  for await (const fragment of modelRequest.text) {
    stream.push(new vscode.ChatResponseMarkdownPart(fragment))
  }
  return Ok(undefined)
}
