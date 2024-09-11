import { isErr, Ok, Result } from '@shared/result'
import * as vscode from 'vscode'

interface ReturnedPrompt {
  agentPrompt: string
  userPrompt: string
}

export const chatReturnFinalChat = async (
  prompt: Result<ReturnedPrompt>,
  model: vscode.LanguageModelChat,
  token: vscode.CancellationToken,
  stream: vscode.ChatResponseStream,
): Promise<Result<undefined>> => {
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
