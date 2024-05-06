import { Language } from '@/utils/sql'
import { PageTitle } from '@/components/PageTitle'
import { Button } from '@/components/Button'
import { CodeBlock } from '@/components/CodeBlock'

interface Props {
  fullSchema: string
  language: Language
}

export const FullSchemaView: React.FC<Props> = ({ fullSchema, language }) => {
  const copyToClipboard = async () => {
    await navigator.clipboard.writeText(fullSchema)
  }

  return (
    <div>
      <div className="pt-5">
        <PageTitle>Schema</PageTitle>
      </div>
      <div className="pt-5">
        <Button label="Copy To Clipboard" onClick={copyToClipboard} />
      </div>
      <div className="whitespace-pre-wrap pt-5">
        <CodeBlock
          code={fullSchema}
          language={{ type: 'sql', variant: language }}
          turnOffCopy
        />
      </div>
    </div>
  )
}
