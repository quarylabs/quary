import {
  FileType,
  Uri,
  WorkspaceFolder,
  FileSystem,
  FileSystemError,
} from 'vscode'
import { Err, isErr, Ok, Result, ResultE, collectResults } from '@shared/result'
import {
  FileSystem as ProtoFileSystem,
  File,
} from '@quary/proto/quary/service/v1/file'

export interface ServicesFiles {
  getProjectRoot: () => Result<Uri>
  listAllFiles: () => Promise<Result<Array<Uri>>>
  readFile: (filePath: Uri) => Promise<Uint8Array>
  readFileBuffer: (filePath: Uri) => Promise<Result<Uint8Array | undefined>>
  doesDirectoryExist: (uri: Uri) => Promise<ResultE<boolean, Error>>
  createDirectory: (uri: Uri) => Promise<void>

  writeFile(uri: Uri, content: Uint8Array): Promise<void>

  getProtoFileSystem: () => Promise<Result<ProtoFileSystem>>
  getStringProjectRoot: () => Promise<Result<string>>
}

export const vsCodeWebFiles = (
  workspacesFolders: readonly WorkspaceFolder[] | undefined,
  fileSystem: FileSystem,
): ServicesFiles => {
  const listAllFiles = async (): Promise<Result<Array<Uri>>> => {
    const workSpaceFolder = getWorkspaceFolder(workspacesFolders)
    if (isErr(workSpaceFolder)) {
      return workSpaceFolder
    }
    return getAllFilesRecursively(workSpaceFolder.value.uri, fileSystem)
  }

  const getProjectRoot = () => {
    const folder = getWorkspaceFolder(workspacesFolders)
    if (isErr(folder)) {
      return folder
    }
    return Ok(folder.value.uri)
  }

  const getStringProjectRoot = async (): Promise<Result<string>> => {
    const projectRoot = getProjectRoot()
    if (isErr(projectRoot)) {
      return projectRoot
    }
    return Ok(projectRoot.value.path)
  }

  return {
    getProjectRoot,
    getStringProjectRoot,
    listAllFiles,
    readFile: async (path) => fileSystem.readFile(path),
    readFileBuffer: async (path) => {
      try {
        const buffer = await fileSystem.readFile(path)
        return Ok(buffer)
      } catch (error) {
        if (error instanceof FileSystemError) {
          if (error.code === 'FileNotFound') {
            return Ok(undefined)
          }
        }
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        return Err(error)
      }
    },
    doesDirectoryExist: async (uri) => {
      try {
        const fileStat = await fileSystem.stat(uri)
        return Ok(fileStat.type === FileType.Directory)
      } catch (error) {
        if (error instanceof FileSystemError) {
          if (error.code === 'FileNotFound') {
            return Ok(false)
          }
        }
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-expect-error
        return Err(error)
      }
    },
    createDirectory: async (uri) => await fileSystem.createDirectory(uri),
    writeFile: async (uri, content) => fileSystem.writeFile(uri, content),
    getProtoFileSystem: async () => {
      const uris = await listAllFiles()
      if (isErr(uris)) {
        return uris
      }
      const filePromises = uris.value.map(async (uri): Promise<File> => {
        const name = uri.path
        const contents = await fileSystem.readFile(uri)
        return { name, contents }
      })
      const files = await Promise.all(filePromises)
      return Ok(
        files.reduce<ProtoFileSystem>(
          (fs, currentValue): ProtoFileSystem => {
            fs.files[currentValue.name] = currentValue
            return fs
          },
          {
            files: {},
          },
        ),
      )
    },
  }
}

/**
 * getWorkspaceFolder returns the first workspace folder or an error if there is none or more than one.
 */
const getWorkspaceFolder = (
  workspacesFolders: readonly WorkspaceFolder[] | undefined,
): Result<WorkspaceFolder> => {
  if (workspacesFolders === undefined) {
    return Err(new Error('expect workSpaceFolders not to be undefined'))
  }
  if (workspacesFolders.length !== 1) {
    return Err(
      new Error(
        `expect workSpaceFolders to have length 1, not ${workspacesFolders.length}`,
      ),
    )
  }
  return Ok(workspacesFolders[0])
}

/**
 * getAllFilesRecursively returns all the files in the folders recursively
 */
const getAllFilesRecursively = async (
  root: Uri,
  fileSystem: FileSystem,
): ReturnTypeForFiles => {
  const directory = await fileSystem.readDirectory(root)
  const promises = await Promise.all(
    directory.map(async ([path, fileType]): ReturnTypeForFiles => {
      switch (fileType) {
        case FileType.Directory: {
          return await getAllFilesRecursively(
            Uri.joinPath(root, path),
            fileSystem,
          )
        }
        case FileType.File: {
          return Ok([Uri.joinPath(root, path)])
        }
        default: {
          return Err(new Error(`unknown file type ${fileType}`))
        }
      }
    }),
  )
  const files = await Promise.all(promises)
  const results = collectResults(files)
  if (isErr(results)) {
    return results
  }
  return Ok(results.value.flat())
}

type ReturnTypeForFiles = Promise<Result<Array<Uri>>>
