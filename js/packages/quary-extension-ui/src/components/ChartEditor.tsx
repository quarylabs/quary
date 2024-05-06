import React from 'react'
import type { ChartEditorData } from '@shared/globalViewState'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { LoadingView } from '../views/LoadingView'
import { ErrorView } from '../views/ErrorView'
import { ChartEditorHeader } from './ChartEditorHeader'
import { Perspective } from './Perspective'

interface Props {
  title: string
  chartResults: ChartEditorData['results']
  chartFile: ChartFile
  allAssets: string[]

  onClickRunQuery: (source: ChartFile['source']) => void
  registerChangeChartFile: (config: ChartFile) => void
}

export const ChartEditor: React.FC<Props> = ({
  chartResults,
  allAssets,
  chartFile,
  onClickRunQuery,
  registerChangeChartFile,
  title,
}) => {
  const RenderedPerspective = () => {
    switch (chartResults.type) {
      case 'loading': {
        return <LoadingView />
      }
      case 'error': {
        return (
          <ErrorView
            error={{
              message: chartResults.errorMessage,
            }}
          />
        )
      }
      case 'not loaded': {
        return <div>Not yet loaded data </div>
      }
      case 'success': {
        return (
          <Perspective
            title={title}
            existingSettings={chartFile.config}
            results={chartResults.queryResult}
            updateConfigListener={(config) => {
              registerChangeChartFile({
                ...chartFile,
                config: JSON.parse(JSON.stringify(config)),
              })
            }}
          />
        )
      }
    }
  }

  return (
    <div className="pt-1">
      <ChartEditorHeader
        data={chartFile.source}
        allAssets={allAssets}
        disabled={chartResults.type === 'loading'}
        onClickRunQuery={onClickRunQuery}
        onChangeSource={(source) => {
          registerChangeChartFile({
            ...chartFile,
            source,
          })
        }}
      />
      <div className="pt-1">
        <RenderedPerspective />
      </div>
    </div>
  )
}
