import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { useCallBackFrontEnd } from '@shared/callBacks.ts'
import { ChartEditor } from '@/components/ChartEditor.tsx'
import { vscode } from '@/utils/VSCodeAPIWrapper.ts'

interface Props {
  chart: ChartEditorData
}

export const ChartEditorView: React.FC<Props> = ({ chart }) => {
  const {
    chartViewChangeHandler,
    chartViewRunQuery,
    chartViewOpenTextEditor,
    chartViewCreateModel,
  } = useCallBackFrontEnd(
    [
      'chartViewChangeHandler',
      'chartViewRunQuery',
      'chartViewOpenTextEditor',
      'chartViewCreateModel',
    ],
    vscode.postMessage,
  )
  const [chartFile, setChartFile] = React.useState(chart.chartFile)

  return (
    <ChartEditor
      onClickCreateModel={chartViewCreateModel}
      title={chart.title}
      chartResults={chart.results}
      chartFile={
        chartFile || {
          tags: [],
          source: {
            $case: 'rawSql',
            rawSql: '',
          },
          config: {},
        }
      }
      registerChangeChartFile={(chartFile) => {
        setChartFile(chartFile)
        chartViewChangeHandler(chartFile)
      }}
      onClickRunQuery={chartViewRunQuery}
      allAssets={chart.allAssets}
      onClickEdit={() => chartViewOpenTextEditor(null)}
    />
  )
}
