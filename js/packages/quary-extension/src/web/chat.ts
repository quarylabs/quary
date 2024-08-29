import * as vscode from 'vscode'
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { chatExplain } from './chatExplain'
import { chatEdit } from './chatEdit'
import { chatGenerate } from './chatGenerate'

const CHAT_ID = 'quary-extension.quary'

export const registerChat = (context: vscode.ExtensionContext) => {
  const chat = vscode.chat.createChatParticipant(CHAT_ID, handler(context))
  // chat.iconPath = 'images/quary-logo.png'

  context.subscriptions.push(chat)
}

// Use gpt-4o since it is fast and high quality. gpt-3.5-turbo and gpt-4 are also available.
const MODEL_SELECTOR: vscode.LanguageModelChatSelector = {
  vendor: 'copilot',
  family: 'gpt-4o',
}

const handler =
  (extensionContext: vscode.ExtensionContext): vscode.ChatRequestHandler =>
  async (
    request: vscode.ChatRequest,
    context: vscode.ChatContext,
    stream: vscode.ChatResponseStream,
    token: vscode.CancellationToken,
  ): Promise<vscode.ChatResult> => {
    const response = await wrappedHandler(
      extensionContext,
      request,
      context,
      stream,
      token,
    )
    if (isErr(response)) {
      // TODO Do better here
      throw response.error
    }
    return response.value
  }

const wrappedHandler = async (
  extensionContext: vscode.ExtensionContext,
  request: vscode.ChatRequest,
  context: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
): Promise<Result<vscode.ChatResult>> => {
  const models = await vscode.lm.selectChatModels(MODEL_SELECTOR)
  if (models.length < 1) {
    return Err({
      code: ErrorCodes.FAILED_PRECONDITION,
      message: 'There are no chat models available for chat.',
    })
  }
  const model = models[0]
  await handleChatRequest(
    extensionContext,
    model,
    request,
    context,
    stream,
    token,
  )
  return Ok({})
}

const Methods: Record<
  string,
  (
    extensionContext: vscode.ExtensionContext,
    model: vscode.LanguageModelChat,
    request: vscode.ChatRequest,
    context: vscode.ChatContext,
    stream: vscode.ChatResponseStream,
    token: vscode.CancellationToken,
  ) => Promise<Result<void>>
> = {
  explain: chatExplain,
  edit: chatEdit,
  generate: chatGenerate,
}

const handleChatRequest = async (
  extensionContext: vscode.ExtensionContext,
  model: vscode.LanguageModelChat,
  request: vscode.ChatRequest,
  context: vscode.ChatContext,
  stream: vscode.ChatResponseStream,
  token: vscode.CancellationToken,
): Promise<Result<void>> => {
  if (request.command) {
    const method = Methods[request.command]
    if (method) {
      return method(extensionContext, model, request, context, stream, token)
    }
    throw new Error('Method not implemented')
  }
  const craftedPrompt = [vscode.LanguageModelChatMessage.User(request.prompt)]
  const modelRequest = await model.sendRequest(craftedPrompt, {}, token)

  for await (const fragment of modelRequest.text) {
    stream.push(new vscode.ChatResponseMarkdownPart(fragment))
  }

  return Ok(undefined)
}
