import { ClipboardDocumentIcon } from '@heroicons/react/20/solid'
import { Language } from '@ui/utils/sql'
import { PageTitle } from '@ui/components/PageTitle'
import { Button } from '@ui/components/ui/button'
import { CodeBlock } from '@ui/components/CodeBlock'

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
        <Button size="sm" onClick={copyToClipboard}>
          <ClipboardDocumentIcon className="mr-2 h-4 w-4" />
          Copy to Clipboard
        </Button>
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
