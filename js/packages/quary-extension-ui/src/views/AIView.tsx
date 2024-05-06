import React from 'react'
import { Dag as DagType } from '@shared/globalViewState'
import { Language } from '@/utils/sql'
import { PageTitle } from '@/components/PageTitle'
import { SectionTitle } from '@/components/SectionTitle'
import { Button } from '@/components/Button'
import { CodeBlock } from '@/components/CodeBlock'
import { Dag } from '@/components/Dag'
import { Warning } from '@/components/Warning'

interface Props {
  aiPrompt: string
  sqlQuery: string
  onClickCreateFile: (contents: string) => void
  language: Language
  projectFile: string
  unknownColumns: string[]
  dag?: DagType
  generatedName?: string
}

export const AIView: React.FC<Props> = ({
  aiPrompt,
  sqlQuery,
  onClickCreateFile,
  language,
  projectFile,
  unknownColumns,
  dag,
  generatedName,
}) => (
  <div>
    <PageTitle>Generated Model</PageTitle>
    <div className="pt-5">
      <SectionTitle>You asked</SectionTitle>
      <div>{aiPrompt}</div>
    </div>
    <div className="pt-5">
      <SectionTitle>AI responded</SectionTitle>
      {generatedName !== undefined ? (
        <div>
          <p>Suggested Model Name: &apos;{generatedName}&apos;</p>
        </div>
      ) : null}
    </div>
    <div className="flex flex-row pt-5">
      <div className="flex-1 px-1">
        <SectionTitle>Sql File</SectionTitle>
        <CodeBlock
          code={sqlQuery}
          language={{ type: 'sql', variant: language }}
        />
        <Button
          label="Create File"
          onClick={() => onClickCreateFile(sqlQuery)}
        />
      </div>
      {projectFile && (
        <div className="flex-1 px-1">
          <SectionTitle>Project File</SectionTitle>
          <CodeBlock code={projectFile} language={{ type: 'yaml' }} />
        </div>
      )}
    </div>
    {unknownColumns.length > 0 ? (
      <div className="pt-5">
        <Warning title="Unrecognised Columns">
          The following columns names could be inferred:{' '}
          {unknownColumns.join(', ')}. This can generally be addressed with AS
          keyword.
        </Warning>
      </div>
    ) : null}
    {dag && (
      <div className="pt-5">
        <SectionTitle>Dependencies</SectionTitle>
        <div>{dag && <Dag dag={dag} />}</div>
      </div>
    )}
  </div>
)
