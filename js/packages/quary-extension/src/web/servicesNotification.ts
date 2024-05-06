// ServicesNotification is an interface to show user notifications
export interface ServicesNotification {
  showMessage(message: string): void
  showErrorMessage(message: string): void
}

export const servicesNotificationVSCode = (
  vscode: typeof import('vscode'),
): ServicesNotification => ({
  showMessage: (message: string) =>
    vscode.window.showInformationMessage(message),
  showErrorMessage: (message: string) =>
    vscode.window.showErrorMessage(message),
})
