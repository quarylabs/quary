import { Meta, StoryObj } from '@storybook/react'
import { ErrorCodes } from '@shared/result'
import { sampleQueryResult } from '../lib/sampleData'
import { ChartEditor } from './ChartEditor'

const meta: Meta<typeof ChartEditor> = {
  component: ChartEditor,
}

export default meta

type Story = StoryObj<typeof ChartEditor>

const title = 'model_a_chart'
const rawSql = {
  $case: 'rawSql' as const,
  rawSql: 'SELECT * FROM table',
}
const assetReference = {
  $case: 'reference' as const,
  reference: {
    name: 'model_a',
  },
}
const success = {
  type: 'success' as const,
  queryResult: sampleQueryResult,
}
const loading = {
  type: 'loading' as const,
}

export const SuccessRawSql: Story = {
  args: {
    title,
    chartFile: {
      tags: [],
      source: rawSql,
      config: {},
    },
    chartResults: success,
  },
}

export const ErrorRawSql: Story = {
  args: {
    ...SuccessRawSql.args,
    chartResults: {
      type: 'error',
      error: {
        code: ErrorCodes.INVALID_ARGUMENT,
        message: 'error',
      },
    },
  },
}

export const NotLoadedRawSql: Story = {
  args: {
    ...SuccessRawSql.args,
    chartResults: {
      type: 'not loaded',
    },
  },
}

export const SuccessAssetReference: Story = {
  args: {
    ...SuccessRawSql.args,
    chartFile: {
      tags: [],
      source: assetReference,
      config: {},
    },
  },
}

export const LoadingAssetReference: Story = {
  args: {
    ...SuccessAssetReference.args,
    chartResults: loading,
  },
}
