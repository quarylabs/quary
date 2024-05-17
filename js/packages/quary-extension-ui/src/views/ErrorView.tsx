import { ErrorCodes, QuaryError, codeToString } from '@shared/result'
import { Warning } from '@/components/Warning'

interface Props {
  error: QuaryError
}

export const ErrorView: React.FC<Props> = ({ error }) => {
  const getAlertContent = (): {
    title: string
    message: string
  } => {
    switch (error.code) {
      case ErrorCodes.INVALID_ARGUMENT: {
        return {
          title: 'Invalid Argument Error',
          message: error.message,
        }
      }
      default: {
        return {
          title: codeToString(error.code),
          message: error.message,
        }
      }
    }
  }

  const alertContent = getAlertContent()
  return (
    <div className="pt-5">
      <Warning title={alertContent.title}>
        <div>
          <p>{alertContent.message}</p>
        </div>
      </Warning>
    </div>
  )
}
