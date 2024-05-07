import { Meta, StoryObj } from '@storybook/react'
import { Warning } from './Warning'

const meta: Meta<typeof Warning> = {
  component: Warning,
}

export default meta

type Story = StoryObj<typeof Warning>

export const Main: Story = {
  args: {
    title: 'Attention needed',
    children:
      'Lorem ipsum dolor sit amet consectetur adipisicing elit. Aliquid pariatur, ipsum similique veniam quo totam eius aperiam dolorum.',
  },
}

export const LongError: Story = {
  args: {
    title: 'Attention needed',
    children: `Error reading .schema file: Failed to await js function: JsValue(EntryNotFound (FileSystemError): Error: ENOENT: no such file or directory, open '/Users/louisjordan/Desktop/analysis/quary_jaffle_shop/models/staging/schema.yaml'
    EntryNotFound (FileSystemError): Error: ENOENT: no such file or directory, open '/Users/louisjordan/Desktop/analysis/quary_jaffle_shop/models/staging/schema.yaml'
        at Function.e (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:149:6084)
        at Object.readFile (/Applications/Visual Studio Code.app/Contents/Resources/app/out/vs/workbench/api/node/extensionHostProcess.js:149:4463))`,
  },
}
