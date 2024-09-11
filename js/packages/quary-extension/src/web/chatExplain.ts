import * as vscode from 'vscode'
import { isErr, Result } from '@shared/result'
import { getServices, preInitSetup } from './services'
import { chatReturnFinalChat } from './chatHelpers'

export const chatExplain = async (
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
  const prompt = await services.rust.returnExplainModelPrompt({
    projectRoot: details.value.projectRoot,
    modelName,
    userPrompt: request.prompt,
  })
  return chatReturnFinalChat(prompt, model, token, stream)
}
