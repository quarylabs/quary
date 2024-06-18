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
      modelName: 'shifts_hours_summary_very_very_long_name',
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
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'b',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'c',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'd',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'e',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'f',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
        {
          name: 'g',
          tags: [],
          filePath: 'a.sql',
          description: 'The following model shows the shifts hours summary.',
          assetType: 1,
        },
      ],
      dag: {
        nodes: [
          { id: 'a', isCached: false },
          { id: 'b', isCached: false },
          { id: 'c', isCached: false },
          { id: 'd', isCached: false },
          { id: 'e', isCached: false },
          { id: 'f', isCached: false },
          { id: 'g', isCached: false },
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
      error: {
        code: 1,
        message: 'Error message',
      },
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
