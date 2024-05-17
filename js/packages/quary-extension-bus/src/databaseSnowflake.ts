import { Err, ErrorCodes, isErr, Ok, Result } from './result'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { columnsValuesToQueryResult } from './shared'

export async function makeSnowflakeRequest<T>(
  accessToken: string,
  accountUrl: string,
  body?: object,
): Promise<Result<T>> {
  try {
    const headers = {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${accessToken}`,
    }
    const fetchOptions: RequestInit = { method: 'POST', headers }

    if (body) {
      fetchOptions.body = JSON.stringify(body)
    }
    const response = await fetch(
      `${accountUrl}/api/v2/statements`,
      fetchOptions,
    )

    if (!response.ok) {
      const error = await response.json()
      switch (response.status) {
        case 400:
          return Err({
            code: ErrorCodes.INVALID_ARGUMENT,
            message: `Invalid argument in call to Snowflake API, ${JSON.stringify(error)}`,
          })
        default: {
          return Err({
            code: ErrorCodes.UNKNOWN,
            message: `HTTP error: ${response.status}\nDetails: ${error.message}`,
          })
        }
      }
    }
    const jsonResponse = await response.json()
    return Ok(jsonResponse as T)
  } catch (error) {
    const errorMessage =
      error instanceof Error ? error.message : 'Unknown error occurred'
    return Err({
      code: ErrorCodes.UNKNOWN,
      message: `Failed to make Snowflake request: ${errorMessage}`,
    })
  }
}

export async function snowflakeRunStatement(
  accessToken: string,
  accountUrl: string,
  database: string,
  schema: string,
  warehouse: string,
  statement: string,
): Promise<Result<QueryResult>> {
  const body = {
    statement: statement.replace(/`/g, ''),
    database,
    schema,
    warehouse,
  }
  const runStatementResponse = await makeSnowflakeRequest(
    accessToken,
    accountUrl,
    body,
  )
  if (isErr(runStatementResponse)) {
    return runStatementResponse
  }
  const { resultSetMetaData, data: values } = runStatementResponse.value as {
    resultSetMetaData: {
      rowType: {
        name: string
        type: string
      }[]
    }
    data: string[][]
  }

  const columns = resultSetMetaData.rowType.map((row) => ({
    column: row.name,
    type: row.type,
  }))
  const out = columnsValuesToQueryResult({ columns, values })
  return Ok(out)
}
