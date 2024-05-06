import { Meta, StoryObj } from '@storybook/react'
import { TablesView } from './TablesView'

const meta: Meta<typeof TablesView> = {
  component: TablesView,
}

export default meta

type Story = StoryObj<typeof TablesView>

export const Primary: Story = {
  args: {
    tables: [...Array(10).keys()].map((i) => `table_${i}`),
    views: [...Array(10).keys()].map((i) => `view_${i}`),
  },
}
