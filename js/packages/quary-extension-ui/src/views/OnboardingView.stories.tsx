import { Meta, StoryObj } from '@storybook/react'
import { DatabaseOnboardingOptions } from '@shared/globalViewState'
import { OnboardingView } from './OnboardingView'

const meta: Meta<typeof OnboardingView> = {
  component: OnboardingView,
}

export default meta

type Story = StoryObj<typeof OnboardingView>

export const Init: Story = {
  args: {
    states: {
      type: 'init',
    },
  },
}

export const BigQueryLoading: Story = {
  args: {
    states: {
      type: 'listSourcesLoading',
      database: {
        type: DatabaseOnboardingOptions.BigQuery,
        token: 'token',
      },
    },
  },
}

export const SqliteLoading: Story = {
  args: {
    states: {
      type: 'listSourcesLoading',
      database: {
        type: DatabaseOnboardingOptions.Sqlite,
        path: 'db.sqlite',
      },
    },
  },
}

export const SqliteInMemoryLoading: Story = {
  args: {
    states: {
      type: 'listSourcesLoading',
      database: {
        type: DatabaseOnboardingOptions.SQLiteInMemory,
      },
    },
  },
}

export const BigQueryError: Story = {
  args: {
    states: {
      type: 'listSourcesError',
      database: {
        type: 'bigQuery',
        token: 'token',
      },
      error: 'error',
    },
  },
}

export const SqliteError: Story = {
  args: {
    states: {
      type: 'listSourcesError',
      database: {
        type: DatabaseOnboardingOptions.Sqlite,
        path: '/path/to/sqlite.db',
      },
      error: 'error',
    },
  },
}

export const SqliteInMemoryError: Story = {
  args: {
    states: {
      type: 'listSourcesError',
      database: {
        type: DatabaseOnboardingOptions.SQLiteInMemory,
      },
      error: 'error',
    },
  },
}

export const BigQuerySuccess: Story = {
  args: {
    states: {
      type: 'listSourcesSuccess',
      sourceDetails: {
        type: DatabaseOnboardingOptions.BigQuery,
        sources: [
          {
            name: 'stg_employees',
            path: 'quary.transform.stg_employees',
            columns: ['id', 'name', 'age', 'salary'],
          },
          {
            name: 'stg_orders',
            path: 'quary.sources.stg_orders',
            columns: ['id', 'quantity', 'price'],
          },
          {
            name: 'weather_luxembourg',
            path: 'example.weather.weather_luxembourg',
            columns: ['date', 'temperature'],
          },
        ],
        projectsAndDatasets: {
          quary: ['transform', 'sources'],
          example: ['thelook', 'weather', 'demo'],
        },
      },
    },
  },
}

export const BigQuerySuccessVeryLongList: Story = {
  args: {
    states: {
      type: 'listSourcesSuccess',
      sourceDetails: {
        type: DatabaseOnboardingOptions.BigQuery,
        sources: [
          {
            name: 'stg_employees',
            path: 'quary.transform.stg_employees',
            columns: ['id', 'name', 'age', 'salary'],
          },
          {
            name: 'stg_orders',
            path: 'quary.sources.stg_orders',
            columns: ['id', 'quantity', 'price'],
          },
          {
            name: 'weather_luxembourg',
            path: 'example.weather.weather_luxembourg',
            columns: ['date', 'temperature'],
          },
        ],
        projectsAndDatasets: {
          quary: [
            'transform',
            'sources',
            'asdf',
            'asdfasdf',
            'asdfasdfasdf',
            'asdfasdfasdfasdf',
            'asdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdfasdfasdf',
            'asdfasdf',
          ],
          example: [
            'transform',
            'sources',
            'asdf',
            'asdfasdf',
            'asdfasdfasdf',
            'asdfasdfasdfasdf',
            'asdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdfasdf',
            'asdfasdfasdfasdfasdfasdfasdfasdfasdf',
            'asdfasdf',
          ],
        },
      },
    },
  },
}

export const SnowflakeSuccess: Story = {
  args: {
    states: {
      type: 'listSourcesSuccess',
      sourceDetails: {
        type: DatabaseOnboardingOptions.Snowflake,
        sources: [
          {
            name: 'stg_employees',
            path: 'quary.transform.stg_employees',
            columns: ['id', 'name', 'age', 'salary'],
          },
          {
            name: 'stg_orders',
            path: 'quary.sources.stg_orders',
            columns: ['id', 'quantity', 'price'],
          },
          {
            name: 'weather_luxembourg',
            path: 'example.weather.weather_luxembourg',
            columns: ['date', 'temperature'],
          },
        ],
        databasesAndSchemas: {
          quary: ['transform', 'sources'],
          example: ['thelook', 'weather', 'demo'],
        },
      },
    },
  },
}

export const SqliteSuccess: Story = {
  args: {
    states: {
      type: 'listSourcesSuccess',
      sourceDetails: {
        type: DatabaseOnboardingOptions.Sqlite,
        path: '/path/to/sqlite.db',
        sources: [
          {
            name: 'table_1',
            path: 'table_1',
            columns: ['column1', 'column2'],
          },
          {
            name: 'table_2',
            path: 'table_2',
            columns: ['column1', 'column2'],
          },
          {
            name: 'table_3',
            path: 'table_3',
            columns: ['column1', 'column2'],
          },
        ],
      },
    },
  },
}

export const GenerateProjectError: Story = {
  args: {
    states: {
      type: 'generateProjectError',
      error: 'Error generating project',
    },
  },
}
