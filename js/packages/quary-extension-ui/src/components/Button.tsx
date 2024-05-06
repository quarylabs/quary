import { VSCodeButton } from '@vscode/webview-ui-toolkit/react'

interface Props {
  label: string
  onClick: () => void
}

export const Button: React.FC<Props> = ({ label, onClick }) => (
  <VSCodeButton onClick={onClick}>{label}</VSCodeButton>
)
