import * as vscode from 'vscode'
import { QuaryError } from '@shared/result'

export interface ServicesLogger {
  captureException(error: QuaryError): void

  setUser: (user: { id: string; email: string } | null) => void
}

export const servicesLoggerExceptionThrower = (): ServicesLogger => ({
  captureException: (error: QuaryError) => {
    // eslint-disable-next-line no-console
    console.error('Error in servicesLoggerExceptionThrower', error)
    vscode.window.showErrorMessage(`Quary: ${error.message}`)
  },
  setUser: (user) => {
    // eslint-disable-next-line no-console
    console.info('Logger set user to', user)
  },
})
