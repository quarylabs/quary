import { Meta, StoryObj } from '@storybook/react'
import { AIView } from './AIView'

const meta: Meta<typeof AIView> = {
  component: AIView,
}

export default meta

type Story = StoryObj<typeof AIView>

export const Primary: Story = {
  args: {
    aiPrompt: 'Give me the total hours worked by everyone in the company',
    sqlQuery:
      'SELECT employee_id, first_name, last_name, SUM(hours) FROM q.employees GROUP BY employee_id',
    language: 'sqlite',
    unknownColumns: ['SUM(hours)'],
    dag: {
      nodes: [
        {
          id: '1',
          label: 'employees',
        },
        {
          id: '2',
          label: 'employee_hours',
        },
      ],
      edges: [
        {
          from: '1',
          to: '2',
        },
      ],
    },
    generatedName: 'employee_total_hours',
    projectFile: 'models: test',
  },
}
