import { Uri } from 'vscode'
import { Err, ErrorCodes, Ok, Result } from '@shared/result'
import { CLIRPCServiceClientImpl } from '@quary/proto/quary/service/v1/cli_rpc_calls'
import { TerminalExecutor } from './terminalExecutor'

export type Writer = (uri: Uri, content: Uint8Array) => Promise<void>
export type Reader = (uri: Uri) => Promise<Uint8Array>

export const CLIDatabaseService = (terminalExecutor: TerminalExecutor) => {
  const rpc = {
    async request(
      _: string,
      method: string,
      data: Uint8Array,
    ): Promise<Uint8Array> {
      const base64Request = Buffer.from(data).toString('base64')
      const { stdout, stderr, code } = await terminalExecutor.executeCommand(
        'quary',
        ['rpc', method, base64Request || '""'],
      )
      switch (code) {
        case 0: {
          const decodedResponse = Buffer.from(stdout, 'base64')
          return decodedResponse
        }
        case 127: {
          throw new Error(
            'Quary CLI not found/installed, install it here: https://github.com/quarylabs/quary',
          )
        }
        default: {
          throw new Error(`RPC call failed: ${stderr}`)
        }
      }
    },
  }

  return new CLIRPCServiceClientImpl(rpc)
}

export async function CLIDatabaseServiceWrapper<Req, Res>(
  f: (req: Req) => Promise<Res>,
  req: Req,
): Promise<Result<Res>> {
  try {
    const response = await f(req)
    return Ok(response)
  } catch (e) {
    return Err({
      code: ErrorCodes.INTERNAL,
      message: `Failed to execute CLI RPC call: ${e}`,
    })
  }
}
