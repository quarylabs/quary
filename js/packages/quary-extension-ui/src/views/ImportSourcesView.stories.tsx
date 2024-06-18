import { Meta, StoryObj } from '@storybook/react'
import { ImportSourcesView } from './ImportSourcesView'

const meta: Meta<typeof ImportSourcesView> = {
  component: ImportSourcesView,
  argTypes: {
    onSelectSources: { action: 'onSelectSources' },
  },
}

export default meta

type Story = StoryObj<typeof ImportSourcesView>

export const Success: Story = {
  args: {
    state: {
      type: 'success',
      sources: [
        {
          name: 'stg_employees',
          path: 'quary.transform.stg_employees',
          tags: [],
          tests: [],
          columns: [
            { name: 'id', tests: [] },
            { name: 'name', tests: [] },
            { name: 'age', tests: [] },
            { name: 'salary', tests: [] },
          ],
        },
        {
          name: 'stg_orders',
          path: 'quary.sources.stg_orders',
          tags: [],
          tests: [],
          columns: [
            { name: 'id', tests: [] },
            { name: 'quantity', tests: [] },
            { name: 'price', tests: [] },
          ],
        },
        {
          name: 'weather_luxembourg',
          path: 'example.weather.weather_luxembourg',
          tags: [],
          tests: [],
          columns: [
            { name: 'date', tests: [] },
            { name: 'temperature', tests: [] },
          ],
        },
      ],
    },
  },
}

export const SuccessButEmpty: Story = {
  args: {
    state: {
      type: 'success',
      sources: [],
    },
  },
}
export const Loading: Story = {
  args: {
    state: {
      type: 'loading',
    },
  },
}

export const Error: Story = {
  args: {
    state: {
      type: 'error',
      error: 'An error occured',
    },
  },
}

export const ErrorWithRetry: Story = {
  args: {
    state: {
      type: 'error',
      error: 'An error occured',
    },
  },
}
