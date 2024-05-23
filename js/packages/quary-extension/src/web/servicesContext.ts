import * as vscode from 'vscode'
import * as z from 'zod'

const HostDetailsSchema = z.object({
  hostType: z.union([
    z.literal('web'),
    z.literal('desktop'),
    z.literal('codespaces'),
    z.literal('github.dev'),
    z.literal('vscode.dev'),
  ]),
  environment: z.union([z.literal('development'), z.literal('production')]),
  isNewAppInstall: z.boolean(),
  version: z.string(),
})

// extract the inferred type
type HostDetails = z.infer<typeof HostDetailsSchema>

/**
 * ServicesCodeInstanceContext provides functions to give the current context of the VS Code instance.
 */
export interface ServicesCodeInstanceContext {
  getHostDetails(): Promise<HostDetails>
}

export const VSCodeInstanceContext: ServicesCodeInstanceContext = {
  async getHostDetails() {
    const output = {
      hostType: vscode.env.appHost,
      environment: __MODE__,
      // flag set to true if this is the first time the extension is installed
      isNewAppInstall: vscode.env.isNewAppInstall,
      version: __PACKAGE_VERSION__,
    }
    return HostDetailsSchema.parse(output)
  },
}
