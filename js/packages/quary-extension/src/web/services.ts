import * as vscode from 'vscode'
import { isErr, Result, Ok } from '@shared/result'
import { ConnectionConfig } from '@quary/proto/quary/service/v1/connection_config'
import { ServicesDatabase } from './servicesDatabase'
import { ServicesFiles, vsCodeWebFiles } from './servicesFiles'
import {
  ServicesNotification,
  servicesNotificationVSCode,
} from './servicesNotification'
import { databaseFromConfig } from './createDatabaseFromConfig'
import {
  createExtensionStorageService,
  ServicesStorage,
} from './servicesStorage'
import {
  rustWithDatabaseWasmServices,
  rustWithoutDatabaseWasmServices,
} from './servicesRustWasm'

export interface Services {
  fileSystem: ServicesFiles
  database: ServicesDatabase
  notifications: ServicesNotification
  storage: ServicesStorage
  rust: ReturnType<typeof rustWithDatabaseWasmServices>
  connectionConfig: ConnectionConfig
}

export interface PreInitServices {
  fileSystem: ServicesFiles
  notifications: ServicesNotification
  storage: ServicesStorage
  rust: ReturnType<typeof rustWithoutDatabaseWasmServices>
}

export const getServices = async (
  extension: vscode.ExtensionContext,
): Promise<Services> => {
  // use pre-init services to get quary.yaml config if it exists
  const preInitServices = await getPreInitServices(extension)
  const { rust, fileSystem, storage } = preInitServices

  const rootUri = fileSystem.getProjectRoot()
  if (isErr(rootUri)) {
    throw rootUri.error
  }
  const preInitSetupResult = await preInitSetup(preInitServices)
  if (isErr(preInitSetupResult)) {
    throw preInitSetupResult.error
  }
  const { projectRoot } = preInitSetupResult.value

  const projectConfigResult = await rust.get_project_config({
    projectRoot,
  })
  if (isErr(projectConfigResult)) {
    throw projectConfigResult.error
  }
  const { connectionConfig: rawProjectConfig } = projectConfigResult.value

  if (rawProjectConfig === undefined) {
    throw new Error('Conection config is undefined')
  }

  const database = await databaseFromConfig(
    rootUri.value,
    fileSystem,
    fileSystem.writeFile,
    fileSystem.readFile,
    rawProjectConfig,
  )
  if (isErr(database)) {
    // Handle the error, e.g., show an error message to the user and stop execution or retry
    vscode.window.showErrorMessage(
      `Error initializing database: ${database.error.message}`,
    )
    throw database.error
  }

  return {
    rust: rustWithDatabaseWasmServices(
      // merge the database config modified by the extension with the raw project config
      { config: database.value.returnDatabaseConfig(), ...rawProjectConfig },
      fileSystem,
    ),
    fileSystem,
    database: database.value,
    storage,
    notifications: servicesNotificationVSCode(vscode),
    connectionConfig: rawProjectConfig,
  }
}

// services to be used pre quary project initialisation
export const getPreInitServices = async (
  extension: vscode.ExtensionContext,
): Promise<PreInitServices> => {
  const storage = createExtensionStorageService(extension)
  const fileSystem = vsCodeWebFiles(
    vscode.workspace.workspaceFolders,
    vscode.workspace.fs,
  )
  const rootPath = fileSystem.getProjectRoot()
  if (isErr(rootPath)) {
    // Handle the error, e.g., show an error message to the user and stop execution or retry
    vscode.window.showErrorMessage('Quary: ' + rootPath.error.message)
  }
  return {
    rust: rustWithoutDatabaseWasmServices(fileSystem),
    fileSystem,
    storage,
    notifications: servicesNotificationVSCode(vscode),
  }
}

type PreInitSetup = {
  projectRoot: string
}

// safety checks to ensure basic file system is setup correctly
export async function preInitSetup(
  services: PreInitServices | Services,
): Promise<Result<PreInitSetup>> {
  const projectRoot = await services.fileSystem.getStringProjectRoot()
  if (isErr(projectRoot)) {
    return projectRoot
  }
  return Ok({
    projectRoot: projectRoot.value,
  })
}
