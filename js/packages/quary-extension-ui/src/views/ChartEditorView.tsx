import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { ChartEditorHeader } from '@/components/ChartEditorHeader'
import { ChartEditor } from '@/components/ChartEditor'

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
