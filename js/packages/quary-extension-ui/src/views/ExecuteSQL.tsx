import React, { useRef, useState } from 'react'
import {
  PlayIcon,
  ClipboardDocumentListIcon,
  ChartBarSquareIcon,
} from '@heroicons/react/20/solid'
import { DownloadIcon } from '@radix-ui/react-icons'
import { useCallBackFrontEnd } from '@shared/callBacks'
import { SqlDocumentationResultsView } from '@shared/globalViewState'
import { codeToString } from '@shared/result'
import { JSONStruct, JSONValue } from '@shared/jsonValue'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@ui/components/ui/tooltip'
import { Button } from '@ui/components/ui/button'
import { DataTable } from '@ui/components/DataTable'
import { Warning } from '@ui/components/Warning'
import { Input } from '@ui/components/ui/input'
import { Label } from '@ui/components/ui/label'
import { Separator } from '@ui/components/ui/separator'
import { vscode } from '@ui/utils/VSCodeAPIWrapper'
import { LoadingView } from '@ui/views/LoadingView'
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@ui/components/ui/tabs'
import { Perspective } from '@ui/components/Perspective'

interface Props {
  results: SqlDocumentationResultsView
  limit: number | undefined
}

export const ExecuteSQLView: React.FC<Props> = ({ results, limit }) => {
  const [stagedLimit, setStagedLimit] = useState<number | undefined>(limit)

  const {
    executeSQLViewRunQuery,
    executeSQLViewExportCSV,
    executeSQLViewCopyToClipboard,
    executeSQLViewCreateChart,
  } = useCallBackFrontEnd(
    [
      'executeSQLViewRunQuery',
      'executeSQLViewExportCSV',
      'executeSQLViewCopyToClipboard',
      'executeSQLViewCreateChart',
    ],
    vscode.postMessage,
  )
  const loading = results.type === 'loading'

  const chartState = useRef<JSONStruct>({})
  const createChart = async () => {
    await executeSQLViewCreateChart({
      chartSettings: chartState.current,
      model: results.type === 'run' ? results.modelName : '',
    })
  }

  return (
    <>
      <div className="pt-5">
        <TableToolbar
          createChart={createChart}
          loading={loading}
          stagedLimit={stagedLimit}
          setStagedLimit={setStagedLimit}
          reload={executeSQLViewRunQuery}
          exportCSV={() => {
            executeSQLViewExportCSV({
              data:
                results.type === 'run'
                  ? results.results
                  : {
                      columns: [],
                    },
            })
          }}
          copyToClipboard={() => {
            executeSQLViewCopyToClipboard({
              data:
                results.type === 'run'
                  ? results.results
                  : {
                      columns: [],
                    },
            })
          }}
        />
        <Separator className="my-4" />
        <Results
          results={results}
          limit={limit}
          chartSettingsRef={chartState}
        />
      </div>
      <Separator className="my-4" />
    </>
  )
}

const TableToolbar: React.FC<{
  loading: boolean
  stagedLimit: number | undefined
  setStagedLimit: (limit: number | undefined) => void
  reload: (message: { limit: number | undefined }) => void
  exportCSV: () => void
  copyToClipboard: () => void
  createChart: () => void
}> = ({
  createChart,
  loading,
  stagedLimit,
  setStagedLimit,
  reload,
  exportCSV,
  copyToClipboard,
}) => (
  <div className="flex flex-wrap items-end justify-between gap-2">
    <div className="flex items-end gap-2">
      <div>
        <Label htmlFor="limit">LIMIT</Label>
        <Input
          disabled={loading}
          value={stagedLimit}
          onChange={(e) => {
            const value = e.target.value
            setStagedLimit(value === '' ? undefined : Number(value))
          }}
          id="limit"
          type="number"
          min="0"
          placeholder="All results"
        />
      </div>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="default"
              onClick={() => {
                reload({ limit: stagedLimit })
              }}
              disabled={loading}
            >
              <div className="flex items-center gap-1">
                <PlayIcon className="h-5 w-5" />
              </div>
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Reload data with limit</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
    <div className="flex gap-2">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button variant="outline" disabled={loading} onClick={createChart}>
              <ChartBarSquareIcon className="h-5 w-5" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Create Chart</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button variant="outline" disabled={loading} onClick={exportCSV}>
              <DownloadIcon className="h-5 w-5" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Download CSV</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button
              variant="outline"
              onClick={() => {
                copyToClipboard()
              }}
              disabled={loading}
            >
              <div className="flex items-center gap-1">
                <ClipboardDocumentListIcon className="h-5 w-5" />
              </div>
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Copy data to clipboard</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  </div>
)

interface ResultsProps {
  results: SqlDocumentationResultsView
  limit: number | undefined
  chartSettingsRef: React.MutableRefObject<JSONValue>
}

const Results: React.FC<ResultsProps> = ({
  results,
  limit,
  chartSettingsRef,
}) => {
  switch (results.type) {
    case 'error': {
      return (
        <Warning title="Error">
          <p>
            {codeToString(results.error.code)}: {results.error.message}
          </p>
        </Warning>
      )
    }
    case 'loading': {
      return <LoadingView />
    }
    case 'run': {
      return (
        <Tabs defaultValue="table">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="table">Table</TabsTrigger>
            <TabsTrigger value="perspective">Perspective</TabsTrigger>
          </TabsList>
          <TabsContent value="table">
            <DataTable result={results.results} limit={limit} />
          </TabsContent>
          <TabsContent value="perspective">
            <Perspective
              results={results.results}
              updateConfigListener={(chartDefinition) => {
                chartSettingsRef.current = chartDefinition
              }}
            />
          </TabsContent>
        </Tabs>
      )
    }
    default: {
      throw new Error('Unhandled results type')
    }
  }
}
