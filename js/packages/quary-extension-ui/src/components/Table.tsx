import React from 'react'
import {
  Table as TableComponent,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@ui/components/ui/table'

interface Props {
  headers: Array<React.ReactNode>
  rows: Array<Array<React.ReactNode>>
}

export const Table: React.FC<Props> = ({ headers, rows }) => (
  <TableComponent>
    <TableHeader>
      <TableRow>
        {headers.map((header, index) => (
          <TableHead key={index}>{header}</TableHead>
        ))}
      </TableRow>
    </TableHeader>
    <TableBody>
      {rows.length ? (
        rows.map((row, rowIndex) => (
          <TableRow key={rowIndex}>
            {row.map((cell, cellIndex) => (
              <TableCell key={cellIndex}>{cell}</TableCell>
            ))}
          </TableRow>
        ))
      ) : (
        <TableRow>
          <TableCell colSpan={headers.length} className="h-24 text-center">
            No results.
          </TableCell>
        </TableRow>
      )}
    </TableBody>
  </TableComponent>
)

/**
 * TableHeaderWithSubheader component which is used to have a header and subheader in a table as the column headers.
 */
export const TableHeaderWithSubheader: React.FC<{
  header: string
  subHeader?: string | undefined
}> = ({ header, subHeader }) => (
  <div className="flex flex-col">
    <span className={subHeader ? 'mb-1' : ''}>{header}</span>
    {subHeader ? (
      <span className="mb-1 text-xs text-gray-500">{subHeader}</span>
    ) : null}
  </div>
)
