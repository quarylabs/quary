import { spawn } from 'child_process'
import * as vscode from 'vscode'
import * as path from 'path'
import { BinaryManager } from './binaryManager'

export interface CommandProcessResult {
  stdout: string
  stderr: string
  fullOutput: string
  code: number
}

export class TerminalExecutor {
  private outputChannel: vscode.OutputChannel
  private cliBinaryPath: string

  constructor(channelName: string) {
    const binaryManager = new BinaryManager(
      vscode.extensions.getExtension('Quary.quary-extension')?.extensionPath!,
    )

    this.outputChannel = vscode.window.createOutputChannel(channelName)
    this.cliBinaryPath = binaryManager.getBinaryPath()
  }

  public async executeCommand(
    _command: string,
    args?: string[],
  ): Promise<CommandProcessResult> {
    return new Promise<CommandProcessResult>((resolve) => {
      const fullCommandPath = this.cliBinaryPath

      console.log({ fullCommandPath, args })

      // Get the workspace root path
      const workspaceRoot = vscode.workspace.rootPath

      // Set the working directory to the workspace root
      const options = {
        cwd: workspaceRoot,
        shell: true,
      }
      const proc = spawn(fullCommandPath, args || [], options)

      let stdout = ''
      let stderr = ''
      let fullOutput = ''

      proc.stdout.on('data', (data: Buffer) => {
        const chunk = data.toString()
        stdout += chunk
        fullOutput += chunk
        this.outputChannel.append(chunk)
      })

      proc.stderr.on('data', (data: Buffer) => {
        const chunk = data.toString()
        stderr += chunk
        fullOutput += chunk
        this.outputChannel.append(chunk)
      })

      proc.on('close', (code: number) => {
        resolve({ stdout, stderr, fullOutput, code })
      })
    })
  }

  public showOutput() {
    this.outputChannel.show()
  }

  public clearOutput() {
    this.outputChannel.clear()
  }

  public dispose() {
    this.outputChannel.dispose()
  }
}
