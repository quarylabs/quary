import { Meta, StoryObj } from '@storybook/react'
import { ErrorCodes } from '@shared/result'
import { sampleQueryResult } from '../lib/sampleData'
import { ExecuteSQLView } from './ExecuteSQL'

const meta: Meta<typeof ExecuteSQLView> = {
  component: ExecuteSQLView,
}

export default meta

type Story = StoryObj<typeof ExecuteSQLView>

export const Main: Story = {
  args: {
    results: {
      modelName: 'model_a',
      type: 'run',
      results: sampleQueryResult,
    },
    limit: 100,
  },
}

export const NoLimit: Story = {
  args: {
    ...Main.args,
    limit: undefined,
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
        code: ErrorCodes.INVALID_ARGUMENT,
        message: 'Error message',
      },
    },
  },
}
