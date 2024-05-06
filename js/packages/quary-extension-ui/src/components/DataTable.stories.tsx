import { Meta, StoryObj } from '@storybook/react'
import {
  QueryResult,
  QueryResultColumn,
} from '@quary/proto/quary/service/v1/query_result'
import { DataTable } from './DataTable'

const meta: Meta<typeof DataTable> = {
  component: DataTable,
}

export default meta

type Story = StoryObj<typeof DataTable>

const headersRowsToQueryResult = ({
  headers,
  rows,
}: {
  headers: string[]
  rows: string[][]
}): QueryResult => {
  const columns: QueryResultColumn[] = headers.map((header, i) => ({
    name: header,
    values: rows.map((row) => row[i]),
    type: 'Example Type',
  }))

  return {
    columns,
  }
}

export const Main: Story = {
  args: {
    result: headersRowsToQueryResult({
      headers: ['Currency', 'Date', 'Sales Amount', 'Product Name', 'Region'],
      rows: [
        ['USD', '2024-01-05', '$1500', 'Widget A', 'USA'],
        ['EUR', '2024-01-04', '€1200', 'Widget B', 'Germany'],
        ['GBP', '2024-01-03', '£800', 'Widget C', 'UK'],
      ],
    }),
  },
}

export const Empty: Story = {
  args: {
    result: headersRowsToQueryResult({
      headers: ['Currency', 'Date', 'Sales Amount', 'Product Name', 'Region'],
      rows: [],
    }),
  },
}
