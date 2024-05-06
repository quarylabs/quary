import { Meta, StoryObj } from '@storybook/react'
import { ProgressRing } from './ProgressRing'

const meta: Meta<typeof ProgressRing> = {
  component: ProgressRing,
}

export default meta

type Story = StoryObj<typeof ProgressRing>

export const Primary: Story = {
  args: {},
}
