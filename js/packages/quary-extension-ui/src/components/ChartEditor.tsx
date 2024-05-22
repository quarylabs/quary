import React from 'react'
import { isEqual } from 'lodash'
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
  onClickEdit: () => void
  onClickRunQuery: (chartFile: ChartFile) => void
  onClickCreateModel: (sql: string) => void
  registerChangeChartFile: (config: ChartFile) => void
}

export const ChartEditor: React.FC<Props> = ({
  chartResults,
  allAssets,
  chartFile,
  onClickRunQuery,
  onClickEdit,
  registerChangeChartFile,
  title,
  onClickCreateModel,
}) => (
  <div className="pt-1">
    <WrappedMemoizedChartEditorHeader
      data={chartFile.source}
      allAssets={allAssets}
      disabled={chartResults.type === 'loading'}
      onClickEdit={onClickEdit}
      onClickCreateModel={onClickCreateModel}
      onClickRunQuery={(source) => {
        onClickRunQuery({
          ...chartFile,
          source,
        })
      }}
      onChangeSource={(source) => {
        registerChangeChartFile({
          ...chartFile,
          source,
        })
      }}
    />
    <div className="pt-1">
      <RenderedPerspective
        chartFile={chartFile}
        chartResults={chartResults}
        title={title}
        registerChangeChartFile={registerChangeChartFile}
      />
    </div>
  </div>
)

const RenderedPerspective = ({
  chartFile,
  chartResults,
  title,
  registerChangeChartFile,
}: {
  chartResults: ChartEditorData['results']
  title: string
  chartFile: ChartFile
  registerChangeChartFile: (config: ChartFile) => void
}) => {
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
      if (chartFile.config === undefined) {
        return (
          <Perspective
            title={title}
            existingSettings={{}}
            results={chartResults.queryResult}
            source={chartFile.source}
            updateConfigListener={(config) => {
              registerChangeChartFile({
                ...chartFile,
                config: JSON.parse(JSON.stringify(config)),
              })
            }}
          />
        )
      }
      return (
        <PerspectiveWithMemo
          title={title}
          existingSettings={chartFile.config}
          results={chartResults.queryResult}
          source={chartFile.source}
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

/*
    Re-render the Perspective component only when the source settings or results change.
    This prevents unnecessary re-renders triggered by changes in the config object.
*/
const PerspectiveWithMemo = React.memo(
  Perspective,
  (prevProps, nextProps) =>
    isEqual(prevProps.source, nextProps.source) &&
    isEqual(prevProps.results, nextProps.results),
)

const WrappedMemoizedChartEditorHeader = React.memo(
  ChartEditorHeader,
  (prevProps, nextProps) => {
    if (prevProps.allAssets.length !== nextProps.allAssets.length) {
      return false
    }
    if (prevProps.disabled !== nextProps.disabled) {
      return false
    }
    if (prevProps.data?.$case !== nextProps.data?.$case) {
      if (
        prevProps.data?.$case === 'rawSql' &&
        nextProps?.data?.$case === 'rawSql'
      ) {
        return true
      }
      return (
        prevProps.data?.$case === 'preTemplatedSql' &&
        nextProps?.data?.$case === 'preTemplatedSql'
      )
    }
    return true
  },
)
