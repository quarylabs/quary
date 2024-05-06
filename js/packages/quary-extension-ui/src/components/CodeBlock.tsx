import React from 'react'
import { PrismLight as SyntaxHighlighter } from 'react-syntax-highlighter'
import sql from 'react-syntax-highlighter/dist/esm/languages/prism/sql'
import yaml from 'react-syntax-highlighter/dist/esm/languages/prism/yaml'
import vs from 'react-syntax-highlighter/dist/esm/styles/prism/vs'
import { format } from 'sql-formatter'
import { SqlLanguage } from '@shared/config'
import { Button } from '@/components/Button'
import { copyToClipboard } from '@/utils/clipboard'

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
      out = format(code.trim(), {
        language: language.variant === 'duckdb' ? 'sql' : language.variant,
      })
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
          <Button
            label="Copy"
            onClick={async () => await copyToClipboard(out)}
          />
        </div>
      )}
    </div>
  )
}
