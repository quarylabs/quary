import { Meta, StoryObj } from '@storybook/react'
import { LoadingView } from './LoadingView'

const meta: Meta<typeof LoadingView> = {
  component: LoadingView,
}

export default meta

type Story = StoryObj<typeof LoadingView>

export const Main: Story = {
  args: {},
}
