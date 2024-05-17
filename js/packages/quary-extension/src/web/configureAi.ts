import { Err, ErrorCodes, Ok, Result } from '@shared/result'
import * as vscode from 'vscode'
import { AIConfig } from '@shared/config'
import { Services } from './services'

export const configureAi = async (
  services: Services,
): Promise<Result<AIConfig>> => {
  const aiConfig = services.storage.getAiConfig()
  if (aiConfig) {
    return Ok(aiConfig)
  }
  const apiKey = await vscode.window.showInputBox({
    title: 'Enter OpenAI API Key',
  })
  if (!apiKey) {
    return Err({
      code: ErrorCodes.INVALID_ARGUMENT,
      message: 'No API key provided',
    })
  }
  services.storage.setAiConfig({ type: 'openai', apiKey })
  return Ok({ type: 'openai', apiKey })
}
