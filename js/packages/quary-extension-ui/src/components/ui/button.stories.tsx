import React from 'react'
import { Meta, StoryObj } from '@storybook/react'
import { ChevronRightIcon } from '@heroicons/react/20/solid'
import { Button } from './button'

const meta: Meta<typeof Button> = {
  component: Button,
}

export default meta

type Story = StoryObj<typeof Button>

export const Primary: Story = {
  args: { children: 'Button', variant: 'default' },
}
export const Secondary: Story = {
  args: { children: 'Secondary', variant: 'secondary' },
}
export const Destructive: Story = {
  args: { children: 'Destructive', variant: 'destructive' },
}
export const Outline: Story = {
  args: { children: 'Outline', variant: 'outline' },
}
export const Ghost: Story = {
  args: { children: 'ghost', variant: 'ghost' },
}
export const Link: Story = {
  args: { children: 'Link', variant: 'link' },
}
export const Icon: Story = {
  args: {
    children: <ChevronRightIcon className="h-4 w-4" />,
    variant: 'outline',
    size: 'icon',
  },
}
