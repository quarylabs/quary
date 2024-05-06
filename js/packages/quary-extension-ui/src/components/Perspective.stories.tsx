import { Meta, StoryObj } from '@storybook/react'
import { action } from '@storybook/addon-actions'
import { sampleQueryResult } from '../lib/sampleData'
import { Perspective } from './Perspective'

const meta: Meta<typeof Perspective> = {
  component: Perspective,
}

export default meta

type Story = StoryObj<typeof Perspective>

export const Primary: Story = {
  args: {
    results: sampleQueryResult,
  },
}

export const WithUpdateConfigListener: Story = {
  args: {
    results: sampleQueryResult,
    updateConfigListener: (chartDefinition) => {
      action('updateConfigListener')(chartDefinition)
    },
  },
}

export const WithOpenWithSettings: Story = {
  args: {
    ...Primary.args,
    openWithSettings: true,
  },
}

export const WithTitle: Story = {
  args: {
    ...Primary.args,
    title: 'My Chart',
  },
}

export const WithExistingSettings: Story = {
  args: {
    ...Primary.args,
    openWithSettings: true,
    existingSettings: {
      version: '2.10.0',
      plugin: 'Datagrid',
      plugin_config: {
        columns: {},
        editable: false,
        scroll_lock: false,
      },
      columns_config: {},
      settings: true,
      theme: 'Pro Light',
      title: 'My Chart',
      group_by: [],
      split_by: [],
      columns: ['currency', 'value', 'date_opened', 'date_closed'],
      filter: [],
      sort: [['date_opened', 'desc']],
      expressions: {},
      aggregates: {},
    },
  },
}
