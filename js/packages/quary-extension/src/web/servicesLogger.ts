import * as Sentry from '@sentry/browser'
import * as vscode from 'vscode'

export interface ServicesLogger {
  captureException(error: Error): void

  setUser: (user: { id: string; email: string } | null) => void
}

export const servicesLoggerExceptionThrower = (): ServicesLogger => ({
  captureException: (error: Error) => {
    // eslint-disable-next-line no-console
    console.error('Error in servicesLoggerExceptionThrower', error)
    vscode.window.showErrorMessage(`Error: ${JSON.stringify(error)}`)
    throw error
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
    captureException(error: Error) {
      vscode.window.showErrorMessage(`Error: ${JSON.stringify(error)}`)
      Sentry.captureException(error)
    },
    setUser(user: { id: string; email: string } | null): void {
      Sentry.setUser(user)
    },
  }
}
