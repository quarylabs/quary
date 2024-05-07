import { Meta, StoryObj } from '@storybook/react'
import { sampleQueryResult } from '../lib/sampleData'
import { Card } from './Card'
import { ChartEditor } from './ChartEditor'

const meta: Meta<typeof Card> = {
  component: ChartEditor,
  argTypes: {
    registerChangeChartFile: { action: 'change chart file' },
    onClickRunQuery: { action: 'run query' },
    onClickEdit: { action: 'edit' },
  },
}

export default meta

type Story = StoryObj<typeof ChartEditor>

const title = 'model_a_chart'
const allAssets = ['model_a', 'model_b', 'model_c']
const rawSql = {
  $case: 'rawSql',
  rawSql: 'SELECT * FROM table',
}
const assetReference = {
  $case: 'reference',
  reference: 'model_a',
}
const success = {
  type: 'success',
  queryResults: sampleQueryResult,
}
const loading = {
  type: 'loading',
}

export const SuccessRawSql: Story = {
  args: {
    title,
    chartFile: {
      source: rawSql,
      config: {},
    },
    allAssets,
    chartResults: success,
  },
}

export const ErrorRawSql: Story = {
  args: {
    ...SuccessRawSql.args,
    chartResults: {
      type: 'error',
      error: 'Error message',
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
