import { ExtensionContext, ViewColumn } from 'vscode'
import { isErr, Ok, Result } from '@shared/result'
import { useCallBackBackEnd } from '@shared/callBacks'
import { renderingFunction } from './commandsScaffolding'
import { Services, setup } from './services'

export const importSources =
  (getServices: () => Promise<Services>, extensionContext: ExtensionContext) =>
  async (): Promise<Result<undefined>> => {
    const services = await getServices()

    return await renderingFunction({
      title: 'Import Sources',
      viewColumn: ViewColumn.Active,
      fn: async (setState, panel, extensionContext) => {
        useCallBackBackEnd(
          ['importSources'],
          {
            // TODO Add ability to add custom folder
            importSources: async ({ sources, folderPath }) => {
              await setState({
                type: 'importSources',
                state: {
                  type: 'loading',
                },
              })
              const services = await getServices()
              const setupValues = await setup(services)
              if (isErr(setupValues)) {
                await setState({
                  type: 'importSources',
                  state: {
                    type: 'error',
                    error: setupValues.error.message,
                  },
                })
                return
              }
              const sourcesGenerated = await services.rust.generateSourceFiles({
                projectRoot: setupValues.value.projectRoot,
                sources: sources.map((source) => ({
                  name: source.name,
                  path: source.path,
                  columns: source.columns.map((column) => column.name),
                })),
                folderPath,
              })
              if (isErr(sourcesGenerated)) {
                setState({
                  type: 'importSources',
                  state: {
                    type: 'error',
                    error: sourcesGenerated.error.message,
                  },
                })
                return
              }
              panel.dispose()
              return
            },
          },
          setState,
          panel,
          extensionContext,
        )

        const sources = await services.database.listSources()
        if (isErr(sources)) {
          return sources
        }

        return Ok({
          type: 'importSources',
          state: {
            type: 'success',
            sources: sources.value,
          },
        })
      },
      extensionContext,
    })
  }
