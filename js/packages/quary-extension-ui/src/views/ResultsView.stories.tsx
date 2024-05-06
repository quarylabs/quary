import { Meta, StoryObj } from '@storybook/react'
import {
  sampleQueryResult,
  sampleQueryResultWithTypes,
} from '../lib/sampleData'
import { ResultsView } from './ResultsView'

const meta: Meta<typeof ResultsView> = {
  component: ResultsView,
}

export default meta

type Story = StoryObj<typeof ResultsView>

export const Primary: Story = {
  args: {
    originalSql: `
            WITH raw_potential_outstanding_debts
                     AS (SELECT column1 AS id, column2 AS description, column3 AS currency, column4 AS value, column5 AS date_opened, column6 AS date_closed
            FROM (VALUES ('1', 'tax for sale to uk government', 'GBP', '33406.5', '2022-12-21', ''), ('2', 'tax for sale to us government', 'GBP', '16703.25', '2022-12-21', ''), ('3', 'accountants bill for the US taxes', 'GBP', '15000', '2022-12-21', ''), ('4', 'accountants bill for the UK taxes', 'GBP', '300', '2022-12-21', ''))), stg_potential_outstanding_debts AS (
            SELECT
                id, description, currency, CAST (value AS REAL) AS value, date_opened, CASE
                WHEN length (date_closed) == 0 THEN NULL
                ELSE date_closed
                END AS date_closed
            FROM
                raw_potential_outstanding_debts)
            SELECT *
            FROM (SELECT id,
                         description,
                         currency,
                         value,
                         date_opened,
                         date_closed
                  FROM stg_potential_outstanding_debts
                  WHERE date_closed IS NULL
                     OR date (date_closed) < date ('now')) LIMIT 100
        `,
    results: sampleQueryResult,
    limit: 100,
  },
}

export const ExampleWithTypes: Story = {
  args: {
    originalSql: `
            WITH raw_potential_outstanding_debts
                     AS (SELECT column1 AS id, column2 AS description, column3 AS currency, column4 AS value, column5 AS date_opened, column6 AS date_closed
            FROM (VALUES ('1', 'tax for sale to uk government', 'GBP', '33406.5', '2022-12-21', ''), ('2', 'tax for sale to us government', 'GBP', '16703.25', '2022-12-21', ''), ('3', 'accountants bill for the US taxes', 'GBP', '15000', '2022-12-21', ''), ('4', 'accountants bill for the UK taxes', 'GBP', '300', '2022-12-21', ''))), stg_potential_outstanding_debts AS (
            SELECT
                id, description, currency, CAST (value AS REAL) AS value, date_opened, CASE
                WHEN length (date_closed) == 0 THEN NULL
                ELSE date_closed
                END AS date_closed
            FROM
                raw_potential_outstanding_debts)
            SELECT *
            FROM (SELECT id,
                         description,
                         currency,
                         value,
                         date_opened,
                         date_closed
                  FROM stg_potential_outstanding_debts
                  WHERE date_closed IS NULL
                     OR date (date_closed) < date ('now')) LIMIT 100
        `,
    results: sampleQueryResultWithTypes,
    limit: 100,
  },
}
