/* eslint-disable no-console */
import * as vscode from 'vscode'
import * as Sentry from '@sentry/browser'
import { Analytics } from '@june-so/analytics-node'
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
  servicesLoggerSentry,
} from './servicesLogger'
import { ChartEditorProvider } from './chartCustomEditor'
import { getPreInitServices } from './services'

export const commandName = (s: string): string => 'quary.' + s

async function activateCommands(
  context: vscode.ExtensionContext,
  isProduction: boolean,
  logger: ServicesLogger,
  analytics: Analytics,
) {
  returnCommandsWithLogs(context, isProduction, logger, analytics).forEach(
    ([name, func]) =>
      context.subscriptions.push(
        vscode.commands.registerCommand(commandName(name), func),
      ),
  )
}


const SENTRY_DSN =
  'https://360983d50cb2c46d0d39778ce2a3443e@o4506173297524736.ingest.sentry.io/4506175684673536'
const JUNE_ANALYTICS = '9PbCtSiPLLggvaE5'

export async function activate(context: vscode.ExtensionContext) {
  const hostDetails = await VSCodeInstanceContext.getHostDetails()
  const isProduction = hostDetails.environment === 'production'

  console.info(`starting extension activation with details: ${hostDetails}`)

  const logger = isProduction
    ? servicesLoggerSentry(SENTRY_DSN, hostDetails.version)
    : servicesLoggerExceptionThrower()

  Sentry.setTags(hostDetails)

  const analytics = new Analytics(JUNE_ANALYTICS)

  // Register auth providers
  context.subscriptions.push(
    new AuthenticationProviderQuary(context, logger, analytics),
  )
  context.subscriptions.push(new AuthenticationProviderBigQuery(context))
  context.subscriptions.push(new AuthenticationProviderSnowflake(context))

  // register: Quary Commands
  await activateCommands(context, isProduction, logger, analytics)

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
}

// this method is called when your extension is deactivated
// eslint-disable-next-line @typescript-eslint/no-empty-function
export function deactivate() {}
