interface Props {
  children: string
}

export const SectionTitle: React.FC<Props> = ({ children }) => (
  <h2 className="text-lg font-semibold">{children}</h2>
)
