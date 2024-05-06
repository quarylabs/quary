import { Err, isErr, Ok, Result } from './result'
import {
  BigQueryJobReference,
  BigQueryJobResults,
  BigQueryJobStatus,
} from '@quary/proto/quary/service/v1/connection_response'
import { columnsValuesToQueryResult } from './shared'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'

export async function makeBigQueryRequest<T>(
  accessToken: string,
  url: string,
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' = 'GET',
  body?: object,
): Promise<Result<T>> {
  try {
    const headers = {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${accessToken}`,
    }
    const fetchOptions: RequestInit = { method, headers }

    if (body) {
      fetchOptions.body = JSON.stringify(body)
    }
    const response = await fetch(url, fetchOptions)

    if (!response.ok) {
      return Err(new Error(`HTTP error: ${response.status}`))
    }
    const jsonResponse = await response.json()
    return Ok(jsonResponse as T)
  } catch (error) {
    const errorMessage =
      error instanceof Error ? error.message : 'Unknown error occurred'
    return Err(new Error(`Failed to make BigQuery request: ${errorMessage}`))
  }
}

export const runBigQueryStatement = async (
  accessToken: string,
  query: string,
  projectId: string,
): Promise<Result<QueryResult>> => {
  const payload = {
    configuration: {
      query: {
        query,
        useLegacySql: false,
      },
    },
  }
  const response = await makeBigQueryRequest<{
    jobReference: BigQueryJobReference
  }>(
    accessToken,
    `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/jobs`,
    'POST',
    payload,
  )
  if (isErr(response)) {
    return response
  }

  const {
    jobReference: { location: jobLocation, jobId },
  } = response.value

  // Poll the job for completion
  const jobUrl = `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/jobs/${jobId}?location=${jobLocation}`

  let jobDone = false
  let jobJsonResponse
  while (!jobDone) {
    const jobResponse = await makeBigQueryRequest<{
      status: BigQueryJobStatus
    }>(accessToken, jobUrl)
    if (isErr(jobResponse)) {
      return jobResponse
    }
    jobJsonResponse = jobResponse.value
    jobDone = jobJsonResponse.status.state === 'DONE'

    if (!jobDone) {
      await new Promise((resolve) => setTimeout(resolve, 1000))
    }
  }
  if (!jobJsonResponse) {
    throw new Error('Job response is undefined')
  }
  const resultsUrl = `https://bigquery.googleapis.com/bigquery/v2/projects/${projectId}/queries/${jobId}?location=${jobLocation}`
  const jobResults = await makeBigQueryRequest<BigQueryJobResults>(
    accessToken,
    resultsUrl,
    'GET',
  )
  if (isErr(jobResults)) {
    return jobResults // TODO: improve error handling
  }
  const values = jobResults.value.rows?.map((r) => r.f.map((f) => f.v)) || []
  const columns =
    jobResults.value.schema?.fields?.map((f) => {
      const type = f.mode === 'REPEATED' ? `ARRAY<${f.type}>` : f.type
      return {
        column: f.name,
        type,
      }
    }) || []
  const out = columnsValuesToQueryResult({ columns, values })
  return Ok(out)
}
