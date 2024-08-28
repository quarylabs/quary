import React, { useState } from 'react'
import {
  Dag as DagType,
  SqlDocumentationResultsView,
} from '@shared/globalViewState'
import { ResumeIcon } from '@radix-ui/react-icons'
import {
  CubeIcon,
  PencilSquareIcon,
  ArrowPathIcon,
  PlusCircleIcon,
} from '@heroicons/react/20/solid'
import { Table } from '@quary/proto/quary/service/v1/table'
import { ColumnTest } from '@quary/proto/quary/service/v1/project_file'
import { useCallBackFrontEnd } from '@shared/callBacks'
import { codeToString } from '@shared/result'
import { Button } from '@ui/components/ui/button'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@ui/components/ui/dialog'
import { Label } from '@ui/components/ui/label'
import { Separator } from '@ui/components/ui/separator'
import { Textarea } from '@ui/components/ui/textarea'
import { DataTable } from '@ui/components/DataTable'
import { PageTitle } from '@ui/components/PageTitle'
import { Dag } from '@ui/components/Dag'
import { SectionTitle } from '@ui/components/SectionTitle'
import { ModelDetails } from '@ui/components/ModelDetails'
import { LoadingView } from '@ui/views/LoadingView'
import { vscode } from '@ui/utils/VSCodeAPIWrapper'
import { Warning } from '@ui/components/Warning'
import { DagNodeDataType } from '@ui/components/DagNode'
import { Badge } from '@ui/components/ui/badge'

interface Props {
  modelName: string
  limit?: number
  dag?: DagType
  description?: string
  tags: string[]
  table: Table | null
  results: SqlDocumentationResultsView
  hideCreateSchemaButton: boolean
}

export const DocumentationView: React.FC<Props> = ({
  modelName,
  results,
  limit,
  dag,
  description: activeDescription,
  tags,
  table,
  hideCreateSchemaButton,
}) => {
  const {
    documentationViewRunSqlQuery,
    documentationViewOpenFile,
    documentationViewLoad,
    documentationViewAddToSchema,
    documentationViewUpdateDescription,
    documentationViewAddColumn,
    documentationViewAddColumnTest,
    documentationViewRemoveColumnTest,
    documentationViewUpdateColumnDescription,
    documentationViewRemoveColumn,
  } = useCallBackFrontEnd(
    [
      'documentationViewRunSqlQuery',
      'documentationViewOpenFile',
      'documentationViewLoad',
      'documentationViewAddToSchema',
      'documentationViewUpdateDescription',
      'documentationViewAddColumn',
      'documentationViewRemoveColumn',
      'documentationViewAddColumnTest',
      'documentationViewRemoveColumnTest',
      'documentationViewUpdateColumnDescription',
    ],
    vscode.postMessage,
  )

  const [stagedDescription, setStagedDescription] = useState<string>(
    activeDescription || '',
  )

  const openFile = ({ filePath }: DagNodeDataType) => {
    if (!filePath) {
      return
    }
    documentationViewOpenFile({ filePath })
  }

  return (
    <div>
      <div className="my-2 pt-3">
        <div className="flex justify-between">
          <PageTitle>
            <div className="flex items-center gap-1">
              <CubeIcon className="h-5 w-5" />
              {modelName}
            </div>
          </PageTitle>
          <div className="flex gap-3">
            {!hideCreateSchemaButton && (
              <Button
                onClick={() => {
                  documentationViewAddToSchema(null)
                }}
                variant="default"
                size="sm"
                className="border-dashed"
              >
                <PlusCircleIcon className="mr-2 h-4 w-4" />
                Add to Schema
              </Button>
            )}
            <Button
              variant="outline"
              onClick={() => {
                documentationViewLoad(null)
              }}
            >
              <div className="flex items-center gap-1">
                <ArrowPathIcon className="h-5 w-5" />
              </div>
            </Button>
          </div>
        </div>
      </div>
      <div className="my-2">
        <div className="flex items-center">
          <h3 className="font-semibold leading-none tracking-tight">
            Description
          </h3>
          <Dialog>
            <DialogTrigger>
              <Button variant="link" size="sm">
                <PencilSquareIcon className="h-4 w-4" />
              </Button>
            </DialogTrigger>
            <DialogContent className="">
              <DialogHeader>
                <DialogTitle>
                  <div className="flex items-center gap-1">
                    <CubeIcon className="h-5 w-5" />
                    {modelName}
                  </div>
                </DialogTitle>
                <DialogDescription>Model description</DialogDescription>
              </DialogHeader>
              <Label htmlFor="description" className="sr-only">
                Description
              </Label>
              <Textarea
                id="description"
                placeholder="Enter a description for this model."
                value={stagedDescription}
                onInput={(e) => {
                  setStagedDescription(
                    (e.target as HTMLTextAreaElement).value as string,
                  )
                }}
              />
              <DialogFooter>
                <DialogClose className="flex gap-2">
                  <Button
                    type="button"
                    variant="secondary"
                    onClick={() => {
                      setStagedDescription(activeDescription || '')
                    }}
                  >
                    Cancel
                  </Button>
                  <Button
                    type="button"
                    variant="default"
                    onClick={() => {
                      documentationViewUpdateDescription({
                        description: stagedDescription,
                      })
                    }}
                  >
                    Save
                  </Button>
                </DialogClose>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
        <p>{activeDescription}</p>
      </div>
      {tags.length > 0 ? (
        <div className="my-4 flex items-center gap-4">
          <h3 className="font-semibold leading-none tracking-tight">Tags</h3>
          <div className="flex gap-2">
            {tags.map((tag) => (
              <Badge key={tag} variant="default">
                {tag}
              </Badge>
            ))}
          </div>
        </div>
      ) : null}
      <Separator className="my-2" />
      {table ? (
        <div className="pt-5">
          <SectionTitle>Columns</SectionTitle>
          <ModelDetails
            table={table}
            addColumn={(column) => documentationViewAddColumn({ column })}
            addColumnTest={(column: string, columnTest: ColumnTest) => {
              documentationViewAddColumnTest({ column, columnTest })
            }}
            removeColumnTest={(column: string, columnTest: ColumnTest) => {
              documentationViewRemoveColumnTest({ column, columnTest })
            }}
            addDescription={(column, description) => {
              documentationViewUpdateColumnDescription({
                column,
                description,
              })
            }}
            removeColumn={(column) => {
              documentationViewRemoveColumn({
                column,
              })
            }}
          />
        </div>
      ) : null}

      {dag ? (
        <div className="pt-5">
          <SectionTitle>Dependency Tree</SectionTitle>
          <Dag dag={dag} onNodeDoubleClick={openFile} />
        </div>
      ) : null}
      <div className="pt-5">
        <SectionTitle>Results</SectionTitle>
        <Results
          results={results}
          limit={limit}
          onClickRun={() => documentationViewRunSqlQuery(null)}
        />
      </div>
      <Separator className="my-2" />
    </div>
  )
}

const Results: React.FC<{
  results: SqlDocumentationResultsView
  limit: number | undefined
  onClickRun: () => void
}> = ({ results, limit, onClickRun }) => {
  switch (results.type) {
    case 'error': {
      return (
        <Warning title={`${codeToString(results.error.code)} Error`}>
          <p>{results.error.message}</p>
        </Warning>
      )
    }
    case 'loading': {
      return <LoadingView />
    }
    case 'notYetRun': {
      return (
        <Button onClick={onClickRun} variant="default" size="sm">
          <ResumeIcon className="mr-2 h-4 w-4" />
          Run Query
        </Button>
      )
    }
    case 'run': {
      return <DataTable result={results.results} limit={limit} />
    }
    default: {
      const _exhaustiveCheck: never = results
      throw new Error(`Unhandled results type: ${_exhaustiveCheck}`)
    }
  }
}
