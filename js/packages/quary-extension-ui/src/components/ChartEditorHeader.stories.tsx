import { Meta, StoryObj } from '@storybook/react'
import * as React from 'react'
import { ChartEditorHeader } from './ChartEditorHeader'

const meta: Meta<typeof ChartEditorHeader> = {
  component: ChartEditorHeader,
  argTypes: {
    onChangeSource: { action: 'change source ' },
    onClickRunQuery: { action: 'run query' },
    onClickCreateModel: { action: 'create model' },
    onClickEdit: { action: 'edit' },
  },
}

export default meta

type Story = StoryObj<typeof ChartEditorHeader>

const allAssets = ['model_a', 'model_b', 'model_c']

const WrappedWithState = (
  args: React.ComponentPropsWithRef<typeof ChartEditorHeader>,
) => {
  const [data, setData] = React.useState(args.data)
  return <ChartEditorHeader {...args} data={data} onChangeSource={setData} />
}

export const RawSQL: Story = {
  args: {
    data: {
      $case: 'rawSql',
      rawSql: 'SELECT * FROM table',
    },
    allAssets,
    disabled: false,
  },
  render: WrappedWithState,
}

export const DisabledRawSQL: Story = {
  args: {
    ...RawSQL.args,
    disabled: true,
  },
  render: WrappedWithState,
}

export const TemplatedSQL: Story = {
  args: {
    data: {
      $case: 'preTemplatedSql',
      preTemplatedSql: 'SELECT * FROM q.table',
    },
    allAssets,
    disabled: false,
  },
  render: WrappedWithState,
}

export const DisabledTemplatedSQL: Story = {
  args: {
    ...TemplatedSQL.args,
    disabled: true,
  },
  render: WrappedWithState,
}

export const AssetReference: Story = {
  args: {
    data: {
      $case: 'reference',
      reference: 'model_a',
    },
    allAssets,
    disabled: false,
  },
  render: WrappedWithState,
}

export const DisabledAssetReference: Story = {
  args: {
    ...AssetReference.args,
    disabled: true,
  },
  render: WrappedWithState,
}
