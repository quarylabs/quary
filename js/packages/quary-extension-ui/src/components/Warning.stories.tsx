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
