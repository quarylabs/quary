import { Meta, StoryObj } from '@storybook/react'
import { AddTestDialog } from './AddTestDialog'

const meta: Meta<typeof AddTestDialog> = {
  component: AddTestDialog,
  argTypes: {
    addColumnTest: { action: 'clicked' },
  },
}

export default meta

type Story = StoryObj<typeof AddTestDialog>

export const Primary: Story = {
  args: {},
}
