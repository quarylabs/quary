import { Meta, StoryObj } from '@storybook/react'
import { PlusIcon } from '@heroicons/react/20/solid'
import { Tag } from './Tag'

const meta: Meta<typeof Tag> = {
  component: Tag,
}

export default meta

type Story = StoryObj<typeof Tag>

export const Primary: Story = {
  args: {
    label: 'not_null',
  },
}

export const WithIcon: Story = {
  args: {
    label: 'not_null',
    leftIcon: (
      <div className="h-[20px] w-[20px]">
        <PlusIcon />
      </div>
    ),
  },
}
