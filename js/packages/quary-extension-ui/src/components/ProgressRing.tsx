import { cn } from '@/lib/utils'

interface Props {
  className?: string
}

export const ProgressRing: React.FC<Props> = ({ className }) => (
  <>
    <div
      className={cn(
        className,
        'text-vscode-progressBar-background inline-block animate-spin rounded-full border-[3px] border-current border-t-transparent',
      )}
      aria-label="loading"
    >
      <span className="sr-only">Loading...</span>
    </div>
  </>
)
