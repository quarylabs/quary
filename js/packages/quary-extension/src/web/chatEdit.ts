import * as vscode from 'vscode'
import { isErr, Ok, Result } from '@shared/result'
import { getServices, preInitSetup } from './services'

export const chatEdit = async (
  extensionContext: vscode.ExtensionContext,
  model: vscode.LanguageModelChat,
  request: vscode.ChatRequest,
  _: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
): Promise<Result<void>> => {
  if (request.references.length !== 1) {
    throw new Error(
      'Please provide a single model to explain. You can reference a file by using the #file reference.',
    )
  }
  const reference = request.references[0]
  if (reference.id !== 'vscode.file') {
    throw new Error(
      'Please provide a single model to explain. You can reference a file by using the #file reference.',
    )
  }
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  const file: string = reference.value.path as string
  if (!file.endsWith('.sql')) {
    throw Error(
      'Please provide a single model to explain by referencing the .sql file',
    )
  }
  // Derive model from file name
  const modelName = file.split('/')[file.split('/').length - 1].split('.')[0]

  const services = await getServices(extensionContext)
  const details = await preInitSetup(services)
  if (isErr(details)) {
    return details
  }

  const prompt = await services.rust.returnEditModelPrompt({
    projectRoot: details.value.projectRoot,
    modelName,
    prompt: request.prompt,
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
