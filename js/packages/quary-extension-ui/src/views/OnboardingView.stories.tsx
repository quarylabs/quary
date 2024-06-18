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
      },
    },
  },
}

export const SqliteLoading: Story = {
  args: {
    states: {
      type: 'listSourcesLoading',
      database: {
        type: DatabaseOnboardingOptions.SQLite,
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
        type: DatabaseOnboardingOptions.BigQuery,
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
        type: DatabaseOnboardingOptions.SQLite,
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
        databasesAndSchemas: {
          quary: ['transform', 'sources'],
          example: ['thelook', 'weather', 'demo'],
        },
        config: {
          accountUrl: '',
          clientId: '',
          clientSecret: '',
          role: '',
          warehouse: '',
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
        type: DatabaseOnboardingOptions.SQLite,
        path: '/path/to/sqlite.db',
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
