import React from 'react'
import { PageTitle } from '@ui/components/PageTitle'
import { Table } from '@ui/components/Table'

interface Props {
  tables: Array<string>
  views: Array<string>
}

export const TablesView: React.FC<Props> = ({ tables, views }) => (
  <div>
    <div className="pt-5">
      <PageTitle>Tables & Views</PageTitle>
    </div>
    <div className="flex flex-row flex-wrap pt-5">
      <div className="pr-5">
        <Table headers={['Tables']} rows={tables.map((table) => [table])} />
      </div>
      <div>
        <Table headers={['Views']} rows={views.map((view) => [view])} />
      </div>
    </div>
  </div>
)
