import { WebviewPanel } from 'vscode'
import * as vscode from 'vscode'

// eslint-disable-next-line @typescript-eslint/no-require-imports
const JS_STRING = require('../ui/assets/index.js.txt').default
// eslint-disable-next-line @typescript-eslint/no-require-imports
const CSS_STRING = require('../ui/assets/index.css.txt').default

export const HTML_STRING = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Theme Tokens</title>
  <style>${CSS_STRING}</style>
</head>
<body>
  <div id="root"></div>
  <script type="module">${JS_STRING}</script>
</body>
</html>`

export const createWebViewPanel = ({
  title,
  viewColumn,
}: {
  title: string
  viewColumn?: vscode.ViewColumn
}): WebviewPanel => {
  const panel = vscode.window.createWebviewPanel(
    'quary-extension',
    title,
    {
      viewColumn: viewColumn || vscode.ViewColumn.Two,
      preserveFocus: true,
    },
    {
      enableScripts: true,
    },
  )
  panel.webview.html = HTML_STRING
  return panel
}
