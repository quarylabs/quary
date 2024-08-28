/* eslint-disable camelcase */
import { Err, ErrorCodes, isErr, Ok, Result } from '@shared/result'
import { Uri } from 'vscode'
import {
  RustWithDatabaseServiceClientImpl,
  RustWithoutDatabaseServiceClientImpl,
} from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { TestResults } from '@quary/proto/quary/service/v1/test_results'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { DashboardFile } from '@quary/proto/quary/service/v1/dashboard_file'
import { ServicesDatabase } from '@shared/database'
import {
  add_limit_to_select,
  clean_up,
  initSync,
  parse_chart_file,
  parse_dashboard_file,
  rpc_wrapper_with_database,
  rpc_wrapper_without_database,
  run_model_tests,
  run_tests,
  write_chart_file,
} from '../rust_wasm/quary_wasm_bindgen'
import { ServicesFiles } from './servicesFiles'

// eslint-disable-next-line @typescript-eslint/no-var-requires
const wasm = require('../rust_wasm/quary_wasm_bindgen_bg.wasm')

const init = () => {
  const wasmString1 = wasm.slice('data:application/wasm;base64,'.length)
  const wasmArray = Uint8Array.from(atob(wasmString1), (c) => c.charCodeAt(0))

  initSync(wasmArray)
}

export const rustWithDatabaseWasmServices = (
  projectConfig: ConnectionConfig,
  files: ServicesFiles,
) => {
  init()

  const writefunc = returnWriterFunc(files)
  const fileReader = returnReaderFunc(files)
  const filesLister = returnFileLister(files)

  const rpc = {
    request: async (
      _: string,
      method: string,
      data: Uint8Array,
    ): Promise<Uint8Array> => {
      const config = ConnectionConfig.encode(projectConfig).finish()
      return rpc_wrapper_with_database(
        method,
        config,
        writefunc,
        fileReader,
        filesLister,
        data,
      )
    },
  }
  const client = new RustWithDatabaseServiceClientImpl(rpc)
  return {
    parse_project: wrapper(client.ParseProject),
    render_schema: wrapper(client.RenderSchema),
    list_assets: wrapper(client.ListAssets),
    return_data_for_doc_view: wrapper(client.ReturnDataForDocView),
    return_sql_for_seeds_and_models: wrapper(client.ReturnSQLForSeedsAndModels),
    return_full_sql_for_asset: wrapper(client.ReturnFullSqlForAsset),
    getModelTable: wrapper(client.GetModelTable),
    createModelSchemaEntry: wrapper(client.CreateModelSchemaEntry),
    updateAssetDescription: wrapper(client.UpdateAssetDescription),
    returnSQLForInjectedModel: wrapper(client.ReturnSQLForInjectedModel),
    updateModelSourceColumnDescription: wrapper(
      client.UpdateModelOrSourceColumnDescription,
    ),
    addColumnToModelOrSource: wrapper(client.AddColumnToModelOrSource),
    addColumnTestToModelOrSourceColumnRequest: wrapper(
      client.AddColumnTestToModelOrSourceColumn,
    ),
    returnDefinitionLocationsForSQL: wrapper(
      client.ReturnDefinitionLocationsForSQL,
    ),
    removeColumnTestFromModelOrSourceColumnRequest: wrapper(
      client.RemoveColumnTestFromModelOrSourceColumn,
    ),
    returnDashboardWithSql: wrapper(client.ReturnDashboardWithSql),
    generateSourceFiles: wrapper(client.GenerateSourceFiles),
    run_test: async (
      project_root: string,
      test_runner: 'skip' | 'all',
      run_statement: TestRunner,
    ): Promise<TestResults> => {
      const output = await run_tests(
        test_runner,
        ConnectionConfig.encode(projectConfig).finish(),
        fileReader,
        filesLister,
        run_statement,
        project_root,
      )
      return TestResults.decode(output)
    },
    run_model_test: async (
      project_root: string,
      run_statement: TestRunner,
      model_name: string,
      whether_to_include_model_to_source: boolean,
    ): Promise<TestResults> => {
      const output = await run_model_tests(
        ConnectionConfig.encode(projectConfig).finish(),
        fileReader,
        filesLister,
        run_statement,
        model_name,
        project_root,
        whether_to_include_model_to_source,
      )
      return TestResults.decode(output)
    },
  }
}

/**
 * This function runs tests on the given statement. If the statement is
 * successful, then the function returns ['success_call'] and true/false depending on
 * whether the QueryResult returned is empty or not. If the statement is not successful,
 * then the function returns ['error_call'] and the error message.
 **/
export type TestRunner = (
  statement: string,
) => Promise<['success_call', boolean] | ['error_call', string]>

export const createTestRunner =
  (database: ServicesDatabase): TestRunner =>
  async (statement: string) => {
    try {
      const query = await database.runStatement(statement)
      if (isErr(query)) {
        return ['error_call', query.error.message]
      }
      const columns = query.value.columns
      if (columns.length === 0) {
        return ['success_call', true]
      }
      if (columns[0].values.length === 0) {
        return ['success_call', true]
      }
      return ['success_call', false]
    } catch (e: unknown) {
      if (e instanceof Error) {
        return ['error_call', e.toString()]
      }
      return ['error_call', `${e}`]
    }
  }

const wrapper = function <Req, Res>(
  fn: (request: Req) => Promise<Res>,
): (req: Req) => Promise<Result<Res>> {
  return async (request: Req): Promise<Result<Res>> => {
    try {
      return Ok(await fn(request))
    } catch (e: unknown) {
      if (e instanceof Error) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: e.toString(),
        })
      }
      return Err({
        code: ErrorCodes.INTERNAL,
        message: `${e}`,
      })
    }
  }
}

export const rustWithoutDatabaseWasmServices = (files: ServicesFiles) => {
  init()

  const writefunc = returnWriterFunc(files)
  const fileReader = returnReaderFunc(files)
  const filesLister = returnFileLister(files)

  const rpc = {
    request: async (
      _: string,
      method: string,
      data: Uint8Array,
    ): Promise<Uint8Array> =>
      rpc_wrapper_without_database(
        method,
        writefunc,
        fileReader,
        filesLister,
        data,
      ),
  }
  const client = new RustWithoutDatabaseServiceClientImpl(rpc)
  return {
    init_files: wrapper(client.InitFiles),
    is_path_empty: wrapper(client.IsPathEmpty),
    get_project_config: wrapper(client.GetProjectConfig),
    generate_project_files: wrapper(client.GenerateProjectFiles),
    stringify_project_file: wrapper(client.StringifyProjectFile),
    create_model_chart_file: wrapper(client.CreateModelChartFile),
    add_limit_to_select,
    clean_up,
    write_chart_file_to_yaml: (data: ChartFile): Result<Uint8Array> => {
      try {
        const output = ChartFile.encode(data).finish()
        const yaml = write_chart_file(output)
        return Ok(yaml)
      } catch (e) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: JSON.stringify(e),
        })
      }
    },
    parse_chart_file: (data: Uint8Array): Result<ChartFile> => {
      try {
        const output = parse_chart_file(data)
        const file = ChartFile.decode(output)
        return Ok(file)
      } catch (e: unknown) {
        if (e instanceof Error) {
          return Err({
            code: ErrorCodes.INTERNAL,
            message: e.toString(),
          })
        }
        return Err({
          code: ErrorCodes.INTERNAL,
          message: JSON.stringify(e),
        })
      }
    },
    write_dashboard_file_to_yaml: (data: DashboardFile): Result<Uint8Array> => {
      try {
        const output = DashboardFile.encode(data).finish()
        const yaml = write_chart_file(output)
        return Ok(yaml)
      } catch (e) {
        return Err({
          code: ErrorCodes.INTERNAL,
          message: JSON.stringify(e),
        })
      }
    },
    parse_dashboard_file: (data: Uint8Array): Result<DashboardFile> => {
      try {
        const output = parse_dashboard_file(data)
        const file = DashboardFile.decode(output)
        return Ok(file)
      } catch (e: unknown) {
        if (e instanceof Error) {
          return Err({
            code: ErrorCodes.INTERNAL,
            message: e.toString(),
          })
        }
        return Err({
          code: ErrorCodes.INTERNAL,
          message: JSON.stringify(e),
        })
      }
    },
  }
}

/**
 * This function is used to return a function that can be used to write files. If the locatin asked to be written to is
 * not in the project root, then an error is thrown.
 *
 * If the location is in the project root and within a directory that exists, then the file is written to that location.
 * If the location in the project root and the directory does not exist, then the directory is created and the file is
 * written to that location.
 *
 * @param files
 */
const returnWriterFunc =
  (files: ServicesFiles) =>
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async (location: string, data: Uint8Array): Promise<void> => {
    const projectRoot = files.getProjectRoot()
    if (isErr(projectRoot)) {
      throw new Error(`error getting project root: ${projectRoot.error}`)
    }
    // TODO Quick fix that needs to be fixed properly. The location is not always a valid path.
    if (location.startsWith(projectRoot.value.path)) {
      location = location.substring(projectRoot.value.path.length)
    }
    const uri = Uri.joinPath(projectRoot.value, location)
    // check if not in project root
    if (!uri.fsPath.startsWith(projectRoot.value.fsPath)) {
      throw new Error(
        `cannot write file to location ${location} because it is not in the project root`,
      )
    }
    // check if the directory exists
    let directory = uri.path.substring(
      0,
      uri.path.lastIndexOf('/') === -1 ? 0 : uri.path.lastIndexOf('/'),
    )
    if (directory.startsWith(projectRoot.value.path)) {
      directory = directory.substring(projectRoot.value.path.length)
    }
    const directoryURI = Uri.joinPath(projectRoot.value, directory)
    const directoryForCreation = projectRoot.value.with({
      path: directoryURI.path,
    })
    await files.createDirectory(directoryForCreation)
    return files.writeFile(uri, data)
  }

const returnReaderFunc =
  (files: ServicesFiles) =>
  async (
    location: string,
  ): Promise<['ok', Uint8Array] | ['error', string] | ['not_found']> => {
    const projectRoot = files.getProjectRoot()
    if (isErr(projectRoot)) {
      throw new Error(`error getting project root: ${projectRoot.error}`)
    }
    if (location.startsWith(projectRoot.value.path)) {
      const uri = Uri.joinPath(
        projectRoot.value,
        location.substring(projectRoot.value.path.length),
      )
      return readHelper(files, uri)
    }
    const uri = Uri.joinPath(projectRoot.value, location)
    return readHelper(files, uri)
  }

const readHelper = async (
  files: ServicesFiles,
  uri: Uri,
): Promise<['ok', Uint8Array] | ['error', string] | ['not_found']> => {
  try {
    const data = await files.readFile(uri)
    return ['ok', data]
  } catch (error: unknown) {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    if ('code' in error && error.code === 'FileNotFound') {
      return ['not_found']
    }
    return ['error', `${error}`]
  }
}

const returnFileLister =
  (files: ServicesFiles) =>
  async (location: string): Promise<string[]> => {
    const returnedValues = await files.listAllFiles()
    if (isErr(returnedValues)) {
      throw new Error(`error getting list of files: ${returnedValues.error}`)
    }
    const projectRoot = files.getProjectRoot()
    if (isErr(projectRoot)) {
      throw new Error(`error getting project root: ${projectRoot.error}`)
    }
    const allFiles = returnedValues.value
    const filtered = allFiles.filter((file) => file.path.startsWith(location))
    return filtered.map((value) =>
      value.path.substring(projectRoot.value.path.length),
    )
  }
