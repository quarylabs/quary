import { QueryResult } from '@quary/proto/quary/service/v1/query_result'

export const sampleQueryResult: QueryResult = {
  columns: [
    { name: 'id', values: ['1', '2', '3', '4'] },
    {
      name: 'description',
      values: [
        'tax for sale to uk government',
        'tax for sale to uk government',
        'tax for sale to uk government',
        'tax for sale to uk government',
      ],
    },
    { name: 'currency', values: ['GBP', 'GBP', 'GBP', 'GBP'] },
    { name: 'value', values: ['33406.5', '33406.5', '33406.5', '33406.5'] },
    {
      name: 'date_opened',
      values: ['2022-12-21', '2022-12-21', '2022-12-21', '2022-12-21'],
    },
    { name: 'date_closed', values: ['null', 'null', 'null', 'null'] },
  ],
}

export const sampleQueryResultWithTypes: QueryResult = {
  columns: [
    {
      name: 'id',
      type: 'VARCHAR',
      values: ['1', '2', '3', '4'],
    },
    {
      name: 'description',
      type: 'VARCHAR',
      values: [
        'tax for sale to uk government',
        'tax for sale to uk government',
        'tax for sale to uk government',
        'tax for sale to uk government',
      ],
    },
    {
      name: 'currency',
      type: 'VARCHAR',
      values: ['GBP', 'GBP', 'GBP', 'GBP'],
    },
    {
      name: 'value',
      type: 'REAL',
      values: ['33406.5', '33406.5', '33406.5', '33406.5'],
    },
    {
      name: 'date_opened',
      type: 'DATE',
      values: ['2022-12-21', '2022-12-21', '2022-12-21', '2022-12-21'],
    },
    {
      name: 'date_closed',
      type: 'DATE',
      values: ['null', 'null', 'null', 'null'],
    },
  ],
}
