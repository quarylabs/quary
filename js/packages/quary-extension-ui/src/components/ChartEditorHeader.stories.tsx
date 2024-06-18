import { Meta, StoryObj } from '@storybook/react'
import { ChartEditorHeader } from './ChartEditorHeader'

const meta: Meta<typeof ChartEditorHeader> = {
  component: ChartEditorHeader,
}

export default meta

type Story = StoryObj<typeof ChartEditorHeader>

const assets = ['model_a', 'model_b', 'model_c']

export const RawSQL: Story = {
  args: {
    chartFileSource: {
      $case: 'rawSql',
      rawSql: 'SELECT * FROM table',
    },
    assets,
    disabled: false,
  },
}

export const DisabledRawSQL: Story = {
  args: {
    ...RawSQL.args,
    disabled: true,
  },
}

export const TemplatedSQL: Story = {
  args: {
    chartFileSource: {
      $case: 'preTemplatedSql',
      preTemplatedSql: 'SELECT * FROM q.table',
    },
    assets,
    disabled: false,
  },
}

export const DisabledTemplatedSQL: Story = {
  args: {
    ...TemplatedSQL.args,
    disabled: true,
  },
}

export const AssetReference: Story = {
  args: {
    chartFileSource: {
      $case: 'reference',
      reference: {
        name: 'model_a',
      },
    },
    assets,
    disabled: false,
  },
}

export const DisabledAssetReference: Story = {
  args: {
    ...AssetReference.args,
    disabled: true,
  },
}
