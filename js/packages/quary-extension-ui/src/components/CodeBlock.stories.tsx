import { Meta, StoryObj } from '@storybook/react'
import { CodeBlock } from './CodeBlock'

const meta: Meta<typeof CodeBlock> = {
  component: CodeBlock,
}

export default meta

type Story = StoryObj<typeof CodeBlock>

export const Main: Story = {
  args: {
    code: `SELECT * FROM users WHERE id = 1;`,
    language: {
      type: 'sql',
      variant: 'bigquery',
    },
  },
}
