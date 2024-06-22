/* eslint-disable camelcase */
import * as vscode from 'vscode'
import {
  isErr,
  Result,
  Ok,
  Err,
  collectResults,
  ErrorCodes,
  isQuaryError,
} from '@shared/result'
import { ExtensionContext, QuickPickItem } from 'vscode'
import type { Analytics } from '@june-so/analytics-node'
import { ListAssetsResponse_Asset_AssetType } from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import {
  getPreInitServices,
  getServices,
  preInitSetup,
  Services,
} from './services'
import { getTestRunnerType } from './configTestRunner'
import { testMapper } from './tests'
import { ServicesLogger } from './servicesLogger'
import { CommandFunctionType, renderingFunction } from './commandsScaffolding'
import { DEFAULT_LIMIT_FOR_SELECT } from './defaults'
import { runDocumentationOnModel } from './commandsDocumentation'
import { executeSQLOnModel } from './commandsExecuteSQL'
// eslint-disable-next-line import/no-cycle
import { onboarding } from './commandsOnboarding'
import { importSources } from './commandsImportSources'
import { createTestRunner } from './servicesRustWasm'

export const returnCommands = (
  getServices: () => Promise<Services>,
  extensionContext: ExtensionContext,
) =>
  ({
    onboarding: onboarding(extensionContext),
    importSources: importSources(getServices, extensionContext),
    runTests: async () => {
      //TODO Refactor
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value
      const testRunnner = await getTestRunnerType(services.storage)

      return await renderingFunction({
        title: 'Test Report',
        fn: async () => {
          let mappedTestRunner: undefined | 'skip' | 'all' = undefined
          switch (testRunnner) {
            case TestRunner.TEST_RUNNER_ALL: {
              mappedTestRunner = 'all'
              break
            }
            case TestRunner.TEST_RUNNER_SKIP: {
              mappedTestRunner = 'skip'
              break
            }
          }
          if (mappedTestRunner === undefined) {
            return Err({
              code: ErrorCodes.INTERNAL,
              message: `unknown test runner ${testRunnner}`,
            })
          }

          const runner = createTestRunner(services.database)
          const tests = await services.rust.run_test(
            projectRoot,
            mappedTestRunner,
            runner,
          )

          const mappedTests = collectResults(tests.results.map(testMapper))
          if (isErr(mappedTests)) {
            return Err(mappedTests.error[0])
          }

          return Ok({
            type: 'tests',
            tests: mappedTests.value,
            runner: testRunnner,
          })
        },
        extensionContext,
      })
    },
    runModelTests: async () => {
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value

      // Get the active text editor
      const activeEditor = vscode.window.activeTextEditor
      if (!activeEditor) {
        return Ok(undefined)
      }

      // Get the file name of the active editor
      const filePath = activeEditor.document.uri.fsPath
      const resultActiveFileName = extractModelNameFromFilePath(filePath)
      if (isErr(resultActiveFileName)) {
        return resultActiveFileName
      }
      const activeFileName = resultActiveFileName.value

      // Check if the active file is a model
      const assets = await services.rust.list_assets({
        projectRoot,
        assetsToSkip: {
          charts: true,
        },
      })
      if (isErr(assets)) {
        return assets
      }
      const asset = assets.value.assets.find(
        (asset) => asset.name === activeFileName,
      )
      if (!asset) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: 'Active file is not a model',
        })
      }

      return await renderingFunction({
        title: 'Model Test Report',
        fn: async () => {
          const runner = createTestRunner(services.database)
          const tests = await services.rust.run_model_test(
            projectRoot,
            runner,
            asset.name,
            true, // TODO: Add user selection whether to run model tests against database or not
          )
          const mappedTests = collectResults(tests.results.map(testMapper))
          if (isErr(mappedTests)) {
            return Err(mappedTests.error[0])
          }
          return Ok({
            type: 'tests',
            tests: mappedTests.value,
            runner: TestRunner.TEST_RUNNER_ALL,
          })
        },
        extensionContext,
      })
    },
    statement: async () => {
      const services = await getServices()
      const preQualifier = services.database.returnPreTableQualifier()

      const input = await vscode.window.showInputBox({
        title: 'SQL Statement',
        prompt: 'Enter a SQL statement',
        value:
          preQualifier === ''
            ? 'SELECT * FROM '
            : `SELECT *
                           FROM ${preQualifier}.`,
      })
      if (!input) {
        return Ok(undefined)
      }

      return await renderingFunction({
        title: 'Quary',
        fn: async () => {
          const limit = DEFAULT_LIMIT_FOR_SELECT
          const preInitServices = await getPreInitServices(extensionContext)
          const limitedInput = preInitServices.rust.add_limit_to_select(
            input,
            limit,
          )
          const statement = await services.database.runStatement(limitedInput)
          if (isErr(statement)) {
            return statement
          }
          return Ok({
            type: 'queryResults',
            originalSql: limitedInput,
            results: statement.value,
            limit,
            language: services.database.returnLanguage(),
          })
        },
        extensionContext,
      })
    },
    renderTables: async () => {
      const services = await getServices()

      return await renderingFunction({
        title: 'Tables',
        fn: async () => {
          const [tables, views] = await Promise.all([
            services.database.listTables(),
            services.database.listViews(),
          ])
          if (isErr(tables)) {
            return tables
          }
          if (isErr(views)) {
            return views
          }
          return Ok({
            type: 'tables',
            tables: tables.value.map(({ name }) => name),
            views: views.value.map(({ name }) => name),
          })
        },
        extensionContext,
      })
    },
    run: async () => {
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value

      return await renderingFunction({
        title: 'Project',
        fn: async () => {
          const queries = await services.rust.return_sql_for_seeds_and_models({
            projectRoot,
            dbQualifier: services.database.returnPreTableQualifier(),
          })
          if (isErr(queries)) {
            return queries
          }

          // TODO Need to show this error to people
          if (queries.value.sql.length === 0) {
            return Err({
              code: ErrorCodes.INVALID_ARGUMENT,
              message: 'No queries found',
            })
          }
          for (const query of queries.value.sql) {
            const result = await services.database.runStatement(query)
            if (isErr(result)) {
              return result
            }
          }
          if (queries.value.project === undefined) {
            return Err({
              code: ErrorCodes.INTERNAL,
              message: 'project is undefined in queries',
            })
          }
          return Ok({
            type: 'project',
            project: queries.value.project,
            seedQueries: queries.value.sql,
          })
        },
        extensionContext,
      })
    },
    // TODO: Add test
    // TODO: Add test that it works with dotfiles
    // TODO: Add ignoring of dotfiles
    init: async () => {
      const preInitServices = await getPreInitServices(extensionContext)
      const preInitSetupResult = await preInitSetup(preInitServices)
      if (isErr(preInitSetupResult)) {
        return preInitSetupResult
      }
      const { projectRoot } = preInitSetupResult.value

      const isPathEmptyResult = await preInitServices.rust.is_path_empty({
        projectRoot,
      })
      if (isErr(isPathEmptyResult)) {
        preInitServices.notifications.showErrorMessage(
          `Error checking if folder is empty: ${isPathEmptyResult.error}`,
        )
        return Ok(undefined)
      }
      if (!isPathEmptyResult.value.isEmpty) {
        preInitServices.notifications.showErrorMessage(
          'Directory must be empty to initialise a project.',
        )
        return Ok(undefined)
      }
      const filesToWrite = await preInitServices.rust.init_files({})
      if (isErr(filesToWrite)) {
        preInitServices.notifications.showErrorMessage(
          `Error initialising project: ${filesToWrite.error}`,
        )
        return Ok(undefined)
      }
      preInitServices.notifications.showMessage('Welcome to Quary 🎉')
      return Ok(undefined)
    },
    renderModel: async () => {
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value
      const assets = await services.rust.list_assets({
        projectRoot,
        assetsToSkip: {
          charts: true,
        },
      })
      if (isErr(assets)) {
        return assets
      }
      const quickPicks: QuickPickItem[] = assets.value.assets.map((asset) => {
        let description
        switch (asset.assetType) {
          case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_MODEL:
            description = 'model'
            break
          case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SOURCE:
            description = 'source'
            break
          case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SEED:
            description = 'seed'
            break
          case ListAssetsResponse_Asset_AssetType.ASSET_TYPE_SNAPSHOT:
            description = 'snapshot'
            break
          default:
            throw new Error(
              `assetType is not a string, but is ${JSON.stringify(
                asset.assetType,
              )}`,
            )
        }
        return {
          label: asset.name,
          description,
          detail: asset.description,
        }
      })
      const input = await vscode.window.showQuickPick(quickPicks, {
        title: 'Select a model',
        matchOnDetail: true,
        matchOnDescription: true,
        canPickMany: false,
      })
      if (input === undefined) {
        return Ok(undefined)
      }

      const preInitServices = await getPreInitServices(extensionContext)
      return runDocumentationOnModel(
        input.label,
        services,
        projectRoot,
        extensionContext,
        preInitServices.rust,
      )
    },
    renderFullSchema: async () => {
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value

      return await renderingFunction({
        title: 'Full Schema',
        fn: async () => {
          const schema = await services.rust.render_schema({
            projectRoot,
          })
          if (isErr(schema)) {
            return schema
          }
          return Ok({
            type: 'schema',
            fullSchema: schema.value.schema,
            language: services.database.returnLanguage(),
          })
        },
        extensionContext,
      })
    },
    databaseConfigShow: async () => {
      const services = await getServices()
      return await renderingFunction({
        title: 'Config',
        fn: async () => {
          const config = services.connectionConfig

          return Ok({ type: 'databaseConfigShow', config })
        },
        extensionContext,
      })
    },
    databaseConfigChange: async () => {
      // TODO: Implement functionality to switch database config
      const services = await getServices()
      return await renderingFunction({
        title: 'Config',
        fn: async () => {
          const config = services.connectionConfig

          return Ok({ type: 'databaseConfigShow', config })
        },
        extensionContext,
      })
    },
    renderSources: async () => {
      const services = await getServices()
      return await renderingFunction({
        title: 'Sources',
        fn: async () => {
          const sources = await services.database.listSources()
          if (isErr(sources)) {
            return sources
          }
          const preInitServices = await getPreInitServices(extensionContext)
          const projectFile = await preInitServices.rust.stringify_project_file(
            {
              projectFile: {
                models: [],
                snapshots: [],
                sources: sources.value,
              },
            },
          )
          if (isErr(projectFile)) {
            return projectFile
          }
          return Ok({
            type: 'databaseShowProjectFile',
            projectFile: projectFile.value.stringifiedProjectFile,
          })
        },
        extensionContext,
      })
    },
    openSqlDocumentation: async () => {
      const services = await getServices()

      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value

      // Get the active text editor
      const activeEditor = vscode.window.activeTextEditor
      if (!activeEditor) {
        return Ok(undefined)
      }

      // Get the file name of the active editor
      const filePath = activeEditor.document.uri.fsPath
      const activeFileName = extractModelNameFromFilePath(filePath)
      if (isErr(activeFileName)) {
        return activeFileName
      }

      // Check if the active file is a model
      const assets = await services.rust.list_assets({
        projectRoot,
        assetsToSkip: {
          charts: true,
        },
      })
      if (isErr(assets)) {
        return assets
      }

      const asset = assets.value.assets.find(
        (asset) => asset.name === activeFileName.value,
      )
      if (!asset) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: `Active file is not a model: ${JSON.stringify(activeFileName)}`,
        })
      }
      const preInitServices = await getPreInitServices(extensionContext)
      return runDocumentationOnModel(
        asset.name,
        services,
        projectRoot,
        extensionContext,
        preInitServices.rust,
      )
    },
    executeSQL: async () => {
      const services = await getServices()
      const details = await preInitSetup(services)
      if (isErr(details)) {
        return details
      }
      const { projectRoot } = details.value

      // Get the active text editor
      const activeEditor = vscode.window.activeTextEditor
      if (!activeEditor) {
        return Ok(undefined)
      }

      // Get the file name of the active editor
      const filePath = activeEditor.document.uri.fsPath
      const activeFileName = extractModelNameFromFilePath(filePath)
      if (isErr(activeFileName)) {
        return activeFileName
      }

      // Check if the active file is a model
      const assets = await services.rust.list_assets({
        projectRoot,
        assetsToSkip: {
          charts: true,
        },
      })
      if (isErr(assets)) {
        return assets
      }

      const asset = assets.value.assets.find(
        (asset) => asset.name === activeFileName.value,
      )
      if (!asset) {
        return Err({
          code: ErrorCodes.INVALID_ARGUMENT,
          message: `Active file is not a model: ${JSON.stringify(activeFileName)}`,
        })
      }

      const preInitServices = getPreInitServices(extensionContext)
      return executeSQLOnModel(
        asset.name,
        services,
        projectRoot,
        extensionContext,
        preInitServices.rust,
      )
    },
  }) satisfies Record<string, CommandFunctionType>

export const returnFullCommandName = <
  K extends keyof ReturnType<typeof returnCommands>,
>(
  command: K,
): string => `quary.${command}`

/**
 * returnCommentsWithLogs returns the commands wrapped with logs for visibility and debugging.
 */
export const returnCommandsWithLogs = (
  context: ExtensionContext,
  isProduction: boolean,
  logger: ServicesLogger,
  analytics: Analytics,
): Array<[string, () => Promise<void>]> => {
  const commands = returnCommands(() => getServices(context), context)
  return Object.entries(commands).map(([name, command]) => [
    name,
    async () => {
      try {
        // eslint-disable-next-line no-console
        console.info(`starting command: ${name}`)
        if (isProduction) {
          analytics.track({
            anonymousId: vscode.env.machineId,
            event: `Execute Command: ${name}`,
            properties: {
              environment: vscode.env.appHost,
            },
          })
        }
        const result = await command()
        if (isErr(result)) {
          logger.captureException(result.error)
        }

        // eslint-disable-next-line no-console
        console.info(`finished command: ${name}`)
      } catch (e) {
        if (isQuaryError(e)) {
          return logger.captureException(e)
        }
        return logger.captureException({
          code: ErrorCodes.UNKNOWN,
          message: `Unknown error: ${e}`,
        })
      }
    },
  ])
}

function extractModelNameFromFilePath(filePath: string): Result<string> {
  const pathSegments = filePath.split(/[/\\]/)
  const fileName = pathSegments.pop()
  if (!fileName) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: `No file name found in the path: ${filePath}`,
    })
  }
  const modelName = fileName.replace('.sql', '').replace('.snapshot', '')
  return Ok(modelName)
}
