import { Meta, StoryObj } from '@storybook/react'
import { WelcomeView } from './WelcomeView'

const meta: Meta<typeof WelcomeView> = {
  component: WelcomeView,
}

export default meta

type Story = StoryObj<typeof WelcomeView>

export const General: Story = {}
