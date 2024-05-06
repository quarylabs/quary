import { ProgressRing } from '@/components/ProgressRing'

export const LoadingView: React.FC = () => (
  <div className="flex justify-center pt-10">
    <ProgressRing className="h-20 w-20" />
  </div>
)
