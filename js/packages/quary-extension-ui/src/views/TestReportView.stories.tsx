import { Meta, StoryObj } from '@storybook/react'
import { TestRunner } from '@quary/proto/quary/service/v1/test_runner'
import { TestReportView } from './TestReportView'

const meta: Meta<typeof TestReportView> = {
  component: TestReportView,
}

export default meta

type Story = StoryObj<typeof TestReportView>

export const RunnerAll: Story = {
  args: {
    testRunner: TestRunner.TEST_RUNNER_ALL,
    tests: [
      {
        testName: 'test_shift_first_shift_start_not_null',
        query: 'SELECT * FROM shift_first WHERE shift_start IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_summary_total_shifts_gte',
        query: 'SELECT *\nFROM shifts_summary\nWHERE total_shifts < 0\n',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_summary_percentage_morning_shifts_lte',
        query:
          'SELECT *\nFROM shifts_summary\nWHERE percentage_morning_shifts > 100\n',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_by_month_employee_id_not_null',
        query: 'SELECT * FROM shifts_by_month WHERE employee_id IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_shift_not_null',
        query: 'SELECT * FROM shifts WHERE shift IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shop_locations_shop_name_not_null',
        query: 'SELECT * FROM stg_shop_locations WHERE shop_name IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shifts_shop_id_relationship_stg_shop_locations_id',
        query:
          'SELECT * FROM stg_shifts WHERE shop_id IS NOT NULL AND shop_id NOT IN (SELECT id FROM stg_shop_locations)',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_employees_first_name_not_null',
        query: 'SELECT * FROM stg_employees WHERE first_name IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_summary_employee_id_not_null',
        query: 'SELECT * FROM shifts_summary WHERE employee_id IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shift_first_employee_id_unique',
        query:
          'SELECT * FROM (\n    SELECT employee_id\n    FROM shift_first\n    WHERE employee_id IS NOT NULL\n    GROUP BY employee_id\n    HAVING count(*) > 1\n)',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shop_locations_latitude_gte',
        query: 'SELECT *\nFROM stg_shop_locations\nWHERE latitude < -90\n',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shifts_shift_date_not_null',
        query: 'SELECT * FROM stg_shifts WHERE shift_date IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_employees_last_name_not_null',
        query: 'SELECT * FROM stg_employees WHERE last_name IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shop_locations_id_not_null',
        query: 'SELECT * FROM stg_shop_locations WHERE id IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shift_last_shift_end_not_null',
        query: 'SELECT * FROM shift_last WHERE shift_end IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_shift_start_not_null',
        query: 'SELECT * FROM shifts WHERE shift_start IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shifts_shop_id_not_null',
        query: 'SELECT * FROM stg_shifts WHERE shop_id IS NULL',
        status: { type: 'pass' },
      },
      {
        testName:
          'test_shifts_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM shifts WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'pass' },
      },
      {
        testName: 'test_stg_shifts_employee_id_not_null',
        query: 'SELECT * FROM stg_shifts WHERE employee_id IS NULL',
        status: { type: 'pass' },
      },
      {
        testName:
          'test_stg_shifts_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM stg_shifts WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_shift_relationship_shift_hours_shift',
        query:
          'SELECT * FROM shifts WHERE shift IS NOT NULL AND shift NOT IN (SELECT shift FROM shift_hours)',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shift_hours_shift_not_null',
        query: 'SELECT * FROM shift_hours WHERE shift IS NULL',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shifts_summary_percentage_morning_shifts_gte',
        query:
          'SELECT *\nFROM shifts_summary\nWHERE percentage_morning_shifts < 0\n',
        status: { type: 'pass' },
      },
      {
        testName: 'test_shift_hours_start_time_not_null',
        query: 'SELECT * FROM shift_hours WHERE start_time IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shifts_shift_not_null',
        query: 'SELECT * FROM stg_shifts WHERE shift IS NULL',
        status: { type: 'fail' },
      },
      {
        testName:
          'test_shift_last_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM shift_last WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shifts_shift_accepted_values',
        query:
          "SELECT * FROM stg_shifts WHERE shift IS NOT NULL AND shift NOT IN ('morning','afternoon')",
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_employees_employee_id_unique',
        query:
          'SELECT * FROM (\n    SELECT employee_id\n    FROM stg_employees\n    WHERE employee_id IS NOT NULL\n    GROUP BY employee_id\n    HAVING count(*) > 1\n)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shifts_summary_total_hours_gte',
        query: 'SELECT *\nFROM shifts_summary\nWHERE total_hours < 0\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_longitude_gte',
        query: 'SELECT *\nFROM stg_shop_locations\nWHERE longitude < -180\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_id_unique',
        query:
          'SELECT * FROM (\n    SELECT id\n    FROM stg_shop_locations\n    WHERE id IS NOT NULL\n    GROUP BY id\n    HAVING count(*) > 1\n)',
        status: { type: 'fail' },
      },
      {
        testName:
          'test_shift_first_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM shift_first WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_longitude_not_null',
        query: 'SELECT * FROM stg_shop_locations WHERE longitude IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shifts_employee_id_not_null',
        query: 'SELECT * FROM shifts WHERE employee_id IS NULL',
        status: { type: 'fail' },
      },
      {
        testName:
          'test_shifts_summary_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM shifts_summary WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_latitude_lte',
        query: 'SELECT *\nFROM stg_shop_locations\nWHERE latitude > 90\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shift_first_employee_id_not_null',
        query: 'SELECT * FROM shift_first WHERE employee_id IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shifts_by_month_total_shifts_unique',
        query:
          'SELECT * FROM (\n    SELECT total_shifts\n    FROM shifts_by_month\n    WHERE total_shifts IS NOT NULL\n    GROUP BY total_shifts\n    HAVING count(*) > 1\n)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_manager_id_not_null',
        query: 'SELECT * FROM stg_shop_locations WHERE manager_id IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_latitude_not_null',
        query: 'SELECT * FROM stg_shop_locations WHERE latitude IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_employees_employee_id_not_null',
        query: 'SELECT * FROM stg_employees WHERE employee_id IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shifts_shift_end_not_null',
        query: 'SELECT * FROM shifts WHERE shift_end IS NULL',
        status: { type: 'fail' },
      },
      {
        testName:
          'test_stg_shop_locations_manager_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM stg_shop_locations WHERE manager_id IS NOT NULL AND manager_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shift_last_employee_id_not_null',
        query: 'SELECT * FROM shift_last WHERE employee_id IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_sql__shifts__first_is_always_before_last',
        query:
          'SELECT *\nFROM shifts_summary c\nWHERE c.first_shift > c.last_shift\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_stg_shop_locations_longitude_lte',
        query: 'SELECT *\nFROM stg_shop_locations\nWHERE longitude > 180\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shift_hours_end_time_not_null',
        query: 'SELECT * FROM shift_hours WHERE end_time IS NULL',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shift_hours_shift_unique',
        query:
          'SELECT * FROM (\n    SELECT shift\n    FROM shift_hours\n    WHERE shift IS NOT NULL\n    GROUP BY shift\n    HAVING count(*) > 1\n)',
        status: { type: 'fail' },
      },
      {
        testName:
          'test_shifts_by_month_employee_id_relationship_stg_employees_employee_id',
        query:
          'SELECT * FROM shifts_by_month WHERE employee_id IS NOT NULL AND employee_id NOT IN (SELECT employee_id FROM stg_employees)',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shifts_by_month_total_shifts_gte',
        query: 'SELECT *\nFROM shifts_by_month\nWHERE total_shifts < 0\n',
        status: { type: 'fail' },
      },
      {
        testName: 'test_shift_last_employee_id_unique',
        query:
          'SELECT * FROM (\n    SELECT employee_id\n    FROM shift_last\n    WHERE employee_id IS NOT NULL\n    GROUP BY employee_id\n    HAVING count(*) > 1\n)',
        status: { type: 'fail' },
      },
    ],
  },
}

export const RunnerSkip: Story = {
  args: {
    testRunner: TestRunner.TEST_RUNNER_SKIP,
    // TODO Input tests
    tests: [],
  },
}
