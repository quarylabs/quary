import { Meta, StoryObj } from '@storybook/react'
import { DagNode } from './DagNode'

const meta: Meta<typeof DagNode> = {
  component: DagNode,
}

export default meta

type Story = StoryObj<typeof DagNode>

export const Main: Story = {
  args: {
    data: {
      label: 'test',
      columns: ['id', 'first_name', 'last_name'],
      backGroundLabel: true,
      type: 1,
    },
    turnOffHandles: true,
  },
}
