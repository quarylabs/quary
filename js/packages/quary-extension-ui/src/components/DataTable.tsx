import * as React from 'react'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { queryResultToColumnsValues } from '@shared/shared'
import { Table, TableHeaderWithSubheader } from './Table'

interface Props {
  result: QueryResult
  limit: number | undefined
}

export const DataTable: React.FC<Props> = ({ result, limit }) => {
  const { columns, values } = queryResultToColumnsValues(result)
  return (
    <div className="w-full">
      <Table
        headers={columns.map(({ column, type }) => (
          <TableHeaderWithSubheader
            key={column}
            header={column}
            subHeader={type}
          />
        ))}
        rows={values}
      />
      <div className="pt-5">
        {limit ? (
          <p>
            <strong>Note:</strong> A limit of {limit} was applied.
            {limit > values.length
              ? ` All ${values.length} results are displayed.`
              : ` Showing ${values.length} of potentially more available results.`}
          </p>
        ) : null}
      </div>
    </div>
  )
}
