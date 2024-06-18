import { QueryResult } from '@quary/proto/quary/service/v1/query_result'

function transpose<T>(matrix: T[][]): T[][] {
  // Assuming matrix is not empty and has uniform column lengths across rows
  const rows = matrix.length
  if (rows === 0) {
    return []
  }
  const cols = matrix[0].length
  const newMatrix: T[][] = Array.from({ length: cols }, () =>
    Array.from({ length: rows }),
  )

  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      newMatrix[i][j] = matrix[j][i]
    }
  }

  return newMatrix
}

export function columnsValuesToQueryResult({
  columns,
  values,
}: {
  columns: { column: string; type?: string | undefined }[]
  values: string[][]
}): QueryResult {
  const outValues = transpose(values)
  if (columns.length === 0) {
    return {
      columns: [],
    }
  }
  if (outValues.length === 0) {
    return {
      columns: columns.map(({ column: name, type }) => ({
        name,
        type,
        values: [],
      })),
    }
  }
  return {
    columns: columns.map(({ column, type }, i) => ({
      name: column,
      type,
      values: outValues[i].map((v) => `${v}`),
    })),
  }
}

export function queryResultToColumnsValues({ columns }: QueryResult): {
  columns: { column: string; type: string | undefined }[]
  values: string[][]
} {
  const values = columns.map((column) => column.values)
  const outValues = transpose(values)
  const outColumns = columns.map((column) => ({
    column: column.name,
    type: column.type,
  }))

  return {
    columns: outColumns,
    values: outValues,
  }
}
