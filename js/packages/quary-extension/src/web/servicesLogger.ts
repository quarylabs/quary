import * as Sentry from '@sentry/browser'
import * as vscode from 'vscode'
import { QuaryError, codeToString } from '@shared/result'

export interface ServicesLogger {
  captureException(error: QuaryError): void
  setUser: (user: { id: string; email: string } | null) => void
}

export const servicesLoggerExceptionThrower = (): ServicesLogger => ({
  captureException: (error: QuaryError) => {
    // eslint-disable-next-line no-console
    console.error('Error in servicesLoggerExceptionThrower', error)
    vscode.window.showErrorMessage(
      `${codeToString(error.code)}: ${error.message}`,
    )
  },
  setUser: (user) => {
    // eslint-disable-next-line no-console
    console.info('Logger set user to', user)
  },
})

export const servicesLoggerSentry = (
  dsn: string,
  version: string,
): ServicesLogger => {
  Sentry.init({
    dsn,
    release: version,
  })

  return {
    captureException(error: QuaryError) {
      vscode.window.showErrorMessage(
        `${codeToString(error.code)}: ${error.message}`,
      )
      Sentry.captureException(error)
    },
    setUser(user: { id: string; email: string } | null): void {
      Sentry.setUser(user)
    },
  }
}
