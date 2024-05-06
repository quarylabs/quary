import React from 'react'
import { Meta, StoryObj } from '@storybook/react'
import { Card } from './Card'

const meta: Meta<typeof Card> = {
  component: Card,
}

export default meta

type Story = StoryObj<typeof Card>

export const Primary: Story = {
  args: {
    title: 'Example title',
    children: (
      <div>
        <div>Example card content</div>
      </div>
    ),
  },
}
