import { Meta, StoryObj } from '@storybook/react'
import { ErrorCodes } from '@shared/result'
import { ErrorView } from './ErrorView'

const meta: Meta<typeof ErrorView> = {
  component: ErrorView,
}

export default meta

type Story = StoryObj<typeof ErrorView>

export const Main: Story = {
  args: {
    error: {
      message: 'Error message',
      code: ErrorCodes.INVALID_ARGUMENT,
    },
  },
}
export const ModelReferenceNotFound: Story = {
  args: {
    error: {
      details: {
        type: 'referenceNotFound',
        message: "Model 'ModelName' not found",
      },
      message: 'Error message',
    },
  },
}
