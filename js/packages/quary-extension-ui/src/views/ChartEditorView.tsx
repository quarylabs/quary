import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { useCallBackFrontEnd } from '@shared/callBacks.ts'
import { ChartEditor } from '@/components/ChartEditor.tsx'
import { vscode } from '@/utils/VSCodeAPIWrapper.ts'

interface Props {
  chart: ChartEditorData
}

export const ChartEditorView: React.FC<Props> = ({ chart }) => {
  const { chartViewChangeHandler, chartViewRunQuery } = useCallBackFrontEnd(
    ['chartViewChangeHandler', 'chartViewRunQuery'],
    vscode.postMessage,
  )

  return (
    <ChartEditor
      title={chart.title}
      chartResults={chart.results}
      chartFile={
        chart.chartFile || {
          name: '',
          tags: [],
          source: {
            $case: 'rawSql',
            rawSql: '',
          },
          config: {},
        }
      }
      registerChangeChartFile={chartViewChangeHandler}
      onClickRunQuery={chartViewRunQuery}
      allAssets={chart.allAssets}
    />
  )
}
