import React from 'react'
import { PrismLight as SyntaxHighlighter } from 'react-syntax-highlighter'
import sql from 'react-syntax-highlighter/dist/esm/languages/prism/sql'
import yaml from 'react-syntax-highlighter/dist/esm/languages/prism/yaml'
import vs from 'react-syntax-highlighter/dist/esm/styles/prism/vs'
import { ClipboardDocumentIcon } from '@heroicons/react/20/solid'
import { SqlLanguage } from '@shared/config'
import { Button } from '@ui/components/ui/button'
import { copyToClipboard } from '@ui/utils/clipboard'

interface Props {
  code: string
  language:
    | {
        type: 'sql'
        variant: SqlLanguage
      }
    | {
        type: 'yaml'
      }
  turnOffCopy?: boolean
}

SyntaxHighlighter.registerLanguage('sql', sql)
SyntaxHighlighter.registerLanguage('yaml', yaml)

export const CodeBlock: React.FC<Props> = ({ code, language, turnOffCopy }) => {
  let out: string
  switch (language.type) {
    case 'sql': {
      out = code
      break
    }
    case 'yaml': {
      out = code
      break
    }
    default: {
      throw new Error(`Unknown language type: ${language}`)
    }
  }
  return (
    <div className="relative">
      <div>
        <SyntaxHighlighter language={language.type} style={vs}>
          {out}
        </SyntaxHighlighter>
      </div>
      {turnOffCopy === true ? null : (
        <div className="absolute right-2 top-1">
          <Button size="sm" onClick={async () => await copyToClipboard(out)}>
            <ClipboardDocumentIcon className="mr-2 h-4 w-4" />
            Copy
          </Button>
        </div>
      )}
    </div>
  )
}
