interface Props {
  children: React.ReactNode
}

export const PageTitle: React.FC<Props> = ({ children }) => (
  <h1 className="text-2xl font-semibold">{children}</h1>
)
