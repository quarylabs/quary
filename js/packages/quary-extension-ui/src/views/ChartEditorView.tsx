import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { ChartEditorHeader } from '@ui/components/ChartEditorHeader'
import { ChartEditor } from '@ui/components/ChartEditor'

interface Props {
  data: ChartEditorData
}

export const ChartEditorView: React.FC<Props> = ({ data }) => (
  <>
    <ChartEditorHeader
      chartFileSource={data.chartFile?.source}
      assets={data.allAssets}
      disabled={data.results.type === 'loading'}
    />
    <ChartEditor
      title={data.title}
      chartResults={data.results}
      chartFile={
        data.chartFile || {
          tags: [],
          source: {
            $case: 'rawSql',
            rawSql: '',
          },
          config: {},
        }
      }
    />
  </>
)
