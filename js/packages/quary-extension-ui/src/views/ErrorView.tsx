import { DetailedError } from '@shared/result'
import { Warning } from '@/components/Warning'

interface Props {
  error: DetailedError
}

export const ErrorView: React.FC<Props> = ({ error }) => {
  const getAlertContent = (): {
    title: string
    message: string
  } => {
    if (error.details) {
      switch (error.details.type) {
        case 'modelReferenceNotFound': {
          return {
            title: 'Model reference not found',
            message: error.details.message,
          }
        }
        default: {
          return {
            title: 'Error',
            message: 'An unknonw error occured.',
          }
        }
      }
    }
    return {
      title: 'Error',
      message: error.message,
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
