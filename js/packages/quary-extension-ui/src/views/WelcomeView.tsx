import { Button } from '../components/ui/button'
import { Logo } from '../components/Logo'

interface Props {
  onClick: () => void
}

export const WelcomeView: React.FC<Props> = ({ onClick }) => (
  <div className="flex min-h-screen flex-col items-center justify-center pt-12">
    <Logo className="h-16 w-16" />
    <p className="pt-4 text-2xl font-bold">Welcome to Quary</p>
    <p className="mt-4 max-w-lg text-center">
      To get started, please sign in. The below button will redirect you to our
      website and ask you to sign in.
    </p>
    <div className="mt-4">
      <Button onClick={onClick}>Sign In</Button>
    </div>
  </div>
)
