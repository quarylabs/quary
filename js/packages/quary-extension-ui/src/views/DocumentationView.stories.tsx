import { Meta, StoryObj } from '@storybook/react'
import { DocumentationView } from './DocumentationView'

const meta: Meta<typeof DocumentationView> = {
  component: DocumentationView,
}

export default meta

type Story = StoryObj<typeof DocumentationView>

export const Main: Story = {
  args: {
    results: {
      type: 'run',
      results: {
        columns: [
          {
            name: 'id',
            type: 'VARCHAR',
            values: ['1', '2', '3', '4'],
          },
          {
            name: 'description',
            type: 'VARCHAR',
            values: [
              'tax for sale to uk government',
              'tax for sale to uk government',
              'tax for sale to uk government',
              'tax for sale to uk government',
            ],
          },
          {
            name: 'currency',
            type: 'VARCHAR',
            values: ['GBP', 'GBP', 'GBP', 'GBP'],
          },
          {
            name: 'value',
            type: 'REAL',
            values: ['33406.5', '33406.5', '33406.5', '33406.5'],
          },
          {
            name: 'date_opened',
            type: 'DATE',
            values: ['2022-12-21', '2022-12-21', '2022-12-21', '2022-12-21'],
          },
          {
            name: 'date_closed',
            type: 'DATE',
            values: ['null', 'null', 'null', 'null'],
          },
        ],
      },
    },
    limit: 100,
    modelName: 'shifts_hours_summary_very_very_long_name',
    description: 'The following model shows the shifts hours summary.',
    tags: ['tag1', 'tag2'],
    dag: {
      models: [
        {
          name: 'a',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'b',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'c',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'd',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'e',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'f',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
        {
          name: 'g',
          description: 'The following model shows the shifts hours summary.',
          modelOrSeedOrSource: 1,
        },
      ],
      dag: {
        nodes: [
          { id: 'a', label: 'shifts_hours_summary_very_very_long_name' },
          { id: 'b', label: 'b' },
          { id: 'c', label: 'c' },
          { id: 'd', label: 'd' },
          { id: 'e', label: 'e' },
          { id: 'f', label: 'f' },
          { id: 'g', label: 'g' },
        ],
        edges: [
          { from: 'a', to: 'b' },
          { from: 'c', to: 'b' },
          { from: 'b', to: 'd' },
          { from: 'd', to: 'e' },
          { from: 'f', to: 'e' },
          { from: 'g', to: 'e' },
        ],
      },
    },
  },
}

export const NoDescription: Story = {
  args: {
    ...Main.args,
    description: undefined,
  },
}

export const NoTags: Story = {
  args: {
    ...Main.args,
    tags: [],
  },
}

export const NoDag: Story = {
  args: {
    ...Main.args,
    dag: undefined,
  },
}

export const NoLimit: Story = {
  args: {
    ...Main.args,
    limit: undefined,
  },
}

export const HideCreateSchemaButton: Story = {
  args: {
    ...Main.args,
    hideCreateSchemaButton: true,
  },
}

export const MoreResultsThanLimit: Story = {
  args: {
    ...Main.args,
    limit: 4,
  },
}

export const ResultsLoading: Story = {
  args: {
    ...Main.args,
    results: {
      type: 'loading',
    },
  },
}

export const ResultsError: Story = {
  args: {
    ...Main.args,
    results: {
      type: 'error',
      error: 'Error message',
    },
  },
}

export const ResultsNotYetRun: Story = {
  args: {
    ...Main.args,
    results: {
      type: 'notYetRun',
    },
  },
}
