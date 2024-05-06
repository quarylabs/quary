import React from 'react'

interface Props {
  title: string
  children: React.ReactNode
}

export const Card: React.FC<Props> = ({ title, children }) => (
  <div className="flex h-screen items-center justify-center">
    <div className="bg-vscode-editorWidget-background w-full rounded-lg  shadow-lg sm:w-96">
      <div className="bg-vscode-editorWidget-border rounded-t-lg p-2">
        <h2 className="text-vscode-editor-foreground text-center">{title}</h2>
      </div>
      <div className="p-4">{children}</div>
    </div>
  </div>
)
