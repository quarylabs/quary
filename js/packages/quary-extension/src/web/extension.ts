/* eslint-disable no-console */
import * as vscode from 'vscode'
import { returnCommandsWithLogs } from './commands'
import { VSCodeInstanceContext } from './servicesContext'
import {
  sqlAutocompleteProvider,
  sqlDefinitionProvider,
} from './autoCompleteProvider'
import { AuthenticationProviderQuary } from './authenticationProviderQuary'
import { AuthenticationProviderBigQuery } from './authenticationProviderBigQuery'
import { AuthenticationProviderSnowflake } from './authenticationProviderSnowflake'
import {
  ServicesLogger,
  servicesLoggerExceptionThrower,
} from './servicesLogger'
import { ChartEditorProvider } from './chartCustomEditor'
import { getPreInitServices } from './services'
import { DashboardEditorProvider } from './dashboardCustomEditor'
import { registerChat } from './chat'

export const commandName = (s: string): string => 'quary.' + s

async function activateCommands(
  context: vscode.ExtensionContext,
  logger: ServicesLogger,
) {
  returnCommandsWithLogs(context, logger).forEach(([name, func]) =>
    context.subscriptions.push(
      vscode.commands.registerCommand(commandName(name), func),
    ),
  )
}

export async function activate(context: vscode.ExtensionContext) {
  const hostDetails = await VSCodeInstanceContext.getHostDetails()
  const isProduction = hostDetails.environment === 'production'

  console.info(`starting extension activation with details: ${hostDetails}`)

  const logger = servicesLoggerExceptionThrower()

  // Register auth providers
  context.subscriptions.push(new AuthenticationProviderQuary(context, logger))
  context.subscriptions.push(new AuthenticationProviderBigQuery(context))
  context.subscriptions.push(new AuthenticationProviderSnowflake(context))

  // register: Quary Commands
  await activateCommands(context, logger)

  // show walkthrough if the user has recently installed the extension & has not signed in
  if (hostDetails.isNewAppInstall) {
    await vscode.commands.executeCommand(
      'workbench.action.openWalkthrough',
      'Quary.quary-extension#walkthrough',
    )
  }

  // Register SQL autocompletion provider
  const sqlProvider = sqlAutocompleteProvider(context)
  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider('sql', sqlProvider, '.'),
  )

  // Register SQL definition provider
  const definitionProvider = sqlDefinitionProvider(context)
  context.subscriptions.push(
    vscode.languages.registerDefinitionProvider('sql', definitionProvider),
  )

  // Register chart editor
  const services = await getPreInitServices(context)
  context.subscriptions.push(ChartEditorProvider.register(context, services))
  context.subscriptions.push(
    DashboardEditorProvider.register(context, services),
  )

  // Register chat
  registerChat(context)
}

// this method is called when your extension is deactivated
// eslint-disable-next-line @typescript-eslint/no-empty-function
export function deactivate() {}
