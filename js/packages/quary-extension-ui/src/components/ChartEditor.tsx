import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { useCallBackFrontEnd } from '@shared/callBacks'
import { vscode } from '@ui/utils/VSCodeAPIWrapper'
import { LoadingView } from '../views/LoadingView'
import { ErrorView } from '../views/ErrorView'
import { Perspective } from './Perspective'

interface Props {
  title: string
  chartResults: ChartEditorData['results']
  chartFile: ChartFile
}

export const ChartEditor: React.FC<Props> = ({
  title,
  chartResults,
  chartFile,
}) => {
  const { chartViewMakeChartEdit } = useCallBackFrontEnd(
    ['chartViewMakeChartEdit'],
    vscode.postMessage,
  )
  switch (chartResults.type) {
    case 'loading': {
      return <LoadingView />
    }
    case 'error': {
      return <ErrorView error={chartResults.error} />
    }
    case 'not loaded': {
      return <div>Not yet loaded data </div>
    }
    case 'success': {
      return (
        <Perspective
          title={title}
          existingSettings={chartFile.config || {}}
          results={chartResults.queryResult}
          updateConfigListener={(config) => {
            chartViewMakeChartEdit(JSON.parse(JSON.stringify(config)))
          }}
        />
      )
    }
  }
}
