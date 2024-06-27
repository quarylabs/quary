import { Meta, StoryObj } from '@storybook/react'
import { Dashboard } from './Dashboard'

const meta: Meta<typeof Dashboard> = {
  component: Dashboard,
}

export default meta

type Story = StoryObj<typeof Dashboard>

export const Default: Story = {}
