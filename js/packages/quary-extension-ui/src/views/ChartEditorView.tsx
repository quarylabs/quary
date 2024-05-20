import React, { useCallback, useMemo, useState } from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { useCallBackFrontEnd } from '@shared/callBacks.ts'
import { ChartEditor } from '@/components/ChartEditor.tsx'
import { vscode } from '@/utils/VSCodeAPIWrapper.ts'

interface Props {
  data: ChartEditorData
}

export const ChartEditorView: React.FC<Props> = ({ data }) => {
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

  // the inital state of the chart file
  const [initialChartFile, setInitialChartFile] = useState(data.chartFile)

  // a state used independenetly with highly frequent updates from the ChartFile
  const [stagedChartFile, setStagedChartFile] = useState(data.chartFile)

  const handleRunQuery = useCallback(() => {
    setInitialChartFile(stagedChartFile)
    chartViewRunQuery(stagedChartFile!)
  }, [chartViewRunQuery, stagedChartFile])

  return (
    <ChartEditor
      onClickCreateModel={chartViewCreateModel}
      title={data.title}
      chartResults={data.results}
      chartFile={
        initialChartFile || {
          tags: [],
          source: {
            $case: 'rawSql',
            rawSql: '',
          },
          config: {},
        }
      }
      registerChangeChartFile={(ev) => {
        console.log('register change')
        console.log(ev)
        setStagedChartFile(ev)
        chartViewChangeHandler(ev)
      }}
      onClickRunQuery={handleRunQuery}
      allAssets={data.allAssets}
      onClickEdit={() => chartViewOpenTextEditor(null)}
    />
  )
}
