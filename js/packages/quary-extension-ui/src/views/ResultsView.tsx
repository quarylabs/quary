import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { queryResultToColumnsValues } from '@shared/shared'
import React from 'react'
import { Language } from '@ui/utils/sql'
import { PageTitle } from '@ui/components/PageTitle'
import { Table, TableHeaderWithSubheader } from '@ui/components/Table'
import { SectionTitle } from '@ui/components/SectionTitle'
import { CodeBlock } from '@ui/components/CodeBlock'
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@ui/components/ui/tabs'
import { Perspective } from '@ui/components/Perspective'

interface Props {
  originalSql: string
  results: QueryResult
  limit?: number
  language: Language
}

export const ResultsView: React.FC<Props> = ({
  originalSql,
  results,
  limit,
  language,
}) => {
  const { columns, values } = queryResultToColumnsValues(results)

  return (
    <div>
      <div className="pt-5">
        <PageTitle>Results</PageTitle>
      </div>
      <div className="pt-5">
        <Tabs defaultValue="table">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="table">Table</TabsTrigger>
            <TabsTrigger value="perspective">Perspective</TabsTrigger>
          </TabsList>
          <TabsContent value="table">
            <Table
              headers={columns.map((column) => (
                <TableHeaderWithSubheader
                  key={column.column}
                  header={column.column}
                  subHeader={column.type}
                />
              ))}
              rows={values}
            />
          </TabsContent>
          <TabsContent value="perspective">
            <Perspective results={results} />
          </TabsContent>
        </Tabs>
      </div>
      {limit ? (
        <div className="pt-5">
          <strong>Note:</strong> the LIMIT in the call of {limit}.{' '}
          {limit > values.length - 1
            ? `Given there are more results ${values.length} than the length that should be all the entries.`
            : `Given there are an equal amount of results, there may be more rows.`}
        </div>
      ) : null}
      <div className="pt-10">
        <SectionTitle>SQL Query Run</SectionTitle>
        <CodeBlock
          code={originalSql}
          language={{ type: 'sql', variant: language }}
        />
      </div>
    </div>
  )
}
