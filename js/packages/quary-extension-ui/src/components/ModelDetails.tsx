import { useState } from 'react'
import {
  InfoCircledIcon,
  PlusCircledIcon,
  Pencil2Icon,
  CrossCircledIcon,
  MinusCircledIcon,
  CheckCircledIcon,
} from '@radix-ui/react-icons'
import { ColumnTest } from '@quary/proto/quary/service/v1/project_file'
import {
  RowDescription,
  RowTest,
  Table,
} from '@quary/proto/quary/service/v1/table'
import { Button } from '@ui/components/ui/button'
import { Input } from '@ui/components/ui/input'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@ui/components/ui/tooltip'
import { Badge } from '@ui/components/ui/badge'
import { AddTestDialog } from './AddTestDialog'
import { Table as TableComponent } from './Table'

interface Props {
  table: Table
  addColumn?: (column: string) => void
  removeColumn?: (column: string) => void
  addDescription?: (column: string, description: string) => void
  addColumnTest?: (column: string, columnTest: ColumnTest) => void
  removeColumnTest?: (column: string, columnTest: ColumnTest) => void
}

export const ModelDetails: React.FC<Props> = ({
  table,
  addColumn,
  removeColumn,
  addColumnTest,
  addDescription,
  removeColumnTest,
}) => {
  const tableType = table.tableType
  if (!tableType) {
    throw Error(`No table type provided in ${JSON.stringify(table)}`)
  }
  switch (tableType.$case) {
    case 'present': {
      const table = tableType.present
      const rows = table.rows.map((row) => {
        const rowType = row.row!
        switch (rowType.$case) {
          case 'presentInSqlAndDefinitions': {
            const { title, tests, description } =
              rowType.presentInSqlAndDefinitions
            const addColumnTestForCallback = addColumnTest
              ? (columnTest: ColumnTest) => {
                  addColumnTest(title, columnTest)
                }
              : undefined
            const removeColumnTestForCallback = removeColumnTest
              ? (columnTest: ColumnTest) => {
                  removeColumnTest(title, columnTest)
                }
              : undefined
            const addDescriptionForCallback = addDescription
              ? (description: string) => {
                  addDescription(title, description)
                }
              : undefined
            return RowWrapper({
              title: (
                <PresentInDefinition
                  title={title}
                  onClickRemove={
                    removeColumn ? () => removeColumn(title) : undefined
                  }
                />
              ),
              columnTitle: title,
              tests,
              description,
              addColumnTest: addColumnTestForCallback,
              removeColumnTest: removeColumnTestForCallback,
              addDescription: addDescriptionForCallback,
            })
          }
          case 'presentInDefinitionsButNotRecognisableInSql': {
            const { title, tests, description } =
              rowType.presentInDefinitionsButNotRecognisableInSql
            const addColumnTestForCallback = addColumnTest
              ? (columnTest: ColumnTest) => {
                  addColumnTest(title, columnTest)
                }
              : undefined
            const removeColumnTestForCallback = removeColumnTest
              ? (columnTest: ColumnTest) => {
                  removeColumnTest(title, columnTest)
                }
              : undefined
            const addDescriptionForCallback = addDescription
              ? (description: string) => {
                  addDescription(title, description)
                }
              : undefined
            return RowWrapper({
              title: (
                <PresentInDefinition
                  title={title}
                  onClickRemove={
                    removeColumn ? () => removeColumn(title) : undefined
                  }
                />
              ),
              tests,
              columnTitle: title,
              description,
              addColumnTest: addColumnTestForCallback,
              removeColumnTest: removeColumnTestForCallback,
              addDescription: addDescriptionForCallback,
            })
          }
          case 'missingInDefinitions': {
            const { title, tests, description } = rowType.missingInDefinitions
            const addColumnTestForCallback = addColumnTest
              ? (columnTest: ColumnTest) => {
                  addColumnTest(title, columnTest)
                }
              : undefined
            const removeColumnTestForCallback = removeColumnTest
              ? (columnTest: ColumnTest) => {
                  removeColumnTest(title, columnTest)
                }
              : undefined
            const addDescriptionForCallback = addDescription
              ? (description: string) => {
                  addDescription(title, description)
                }
              : undefined
            const addColumnForCallback = addColumn
              ? () => addColumn(title)
              : undefined
            return RowWrapper({
              title: (
                <InferredTitle title={title} addColumn={addColumnForCallback} />
              ),
              tests,
              columnTitle: title,
              description,
              addColumnTest: addColumnTestForCallback,
              removeColumnTest: removeColumnTestForCallback,
              addDescription: addDescriptionForCallback,
            })
          }
          default:
            throw new Error('Unknown row type')
        }
      })
      return (
        <TableComponent
          headers={['Column', 'Tests', 'Description']}
          rows={rows}
        />
      )
    }
    case 'notPresent': {
      const table = tableType.notPresent
      const rows = table.rows.map(({ title, description, tests }) => {
        const addColumnTestForCallback = addColumnTest
          ? (columnTest: ColumnTest) => {
              addColumnTest(title, columnTest)
            }
          : undefined
        const removeColumnTestForCallback = removeColumnTest
          ? (columnTest: ColumnTest) => {
              removeColumnTest(title, columnTest)
            }
          : undefined
        const addDescriptionForCallback = addDescription
          ? (description: string) => {
              addDescription(title, description)
            }
          : undefined
        const addColumnForCallback = addColumn
          ? () => addColumn(title)
          : undefined
        return RowWrapper({
          title: (
            <InferredTitle title={title} addColumn={addColumnForCallback} />
          ),
          columnTitle: title,
          tests,
          description,
          addDescription: addDescriptionForCallback,
          addColumnTest: addColumnTestForCallback,
          removeColumnTest: removeColumnTestForCallback,
        })
      })
      return (
        <TableComponent
          headers={['Column', 'Tests', 'Description']}
          rows={rows}
        />
      )
    }
    default:
      throw new Error('Unknown table type')
  }
}

const InferredTitle: React.FC<{ title: string; addColumn?: () => void }> = ({
  title,
  addColumn,
}) => (
  <div className="flex items-center justify-center">
    <p className="flex-grow">{title}️</p>
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>
          <Badge onClick={addColumn ? addColumn : () => {}} variant="secondary">
            <div className="flex items-center gap-1">
              {addColumn ? <PlusCircledIcon className="h-3 w-3" /> : null}
              Inferred
            </div>
          </Badge>
        </TooltipTrigger>
        <TooltipContent>
          <p>Add inferred column to model</p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  </div>
)

const PresentInDefinition: React.FC<{
  title: string
  onClickRemove?: () => void
}> = ({ title, onClickRemove }) => (
  <div className="flex items-center justify-center">
    <p className="flex-grow whitespace-nowrap">{title}️</p>
    {onClickRemove ? <RemoveButton onClick={onClickRemove} /> : null}
  </div>
)

const RowWrapper = ({
  title,
  tests,
  description,
  addColumnTest,
  removeColumnTest,
  addDescription,
  columnTitle,
}: {
  columnTitle: string
  title: React.ReactNode
  tests: RowTest[]
  description?: RowDescription
  addColumnTest?: (columnTest: ColumnTest) => void
  removeColumnTest?: (columnTest: ColumnTest) => void
  addDescription?: (description: string) => void
}) => [
  title,
  <TestWrapper
    columnTitle={columnTitle}
    key={`test-${columnTitle}`}
    tests={tests}
    addColumnTest={addColumnTest}
    removeColumnTest={removeColumnTest}
  />,
  <DescriptionWrapper
    key={`description-${columnTitle}`}
    description={description}
    addDescription={addDescription}
  />,
]

const TestBadge = ({
  rowTest,
  addColumnTest,
  removeColumnTest,
}: {
  rowTest: RowTest
  addColumnTest?: (columnTest: ColumnTest) => void
  removeColumnTest?: (columnTest: ColumnTest) => void
}) => {
  const test = rowTest.test!
  switch (test.$case) {
    case 'presentAndNotInferred': {
      const { text, columnTest } = test.presentAndNotInferred
      if (!columnTest) {
        throw Error(
          `No column test for callback provided in ${JSON.stringify(test)}`,
        )
      }
      return (
        <Badge className="flex gap-1">
          {text}
          {removeColumnTest ? (
            <CrossCircledIcon
              onClick={() => {
                removeColumnTest(columnTest)
              }}
              className="h-3 w-3 hover:cursor-pointer"
            />
          ) : null}
        </Badge>
      )
    }
    case 'presentAndInferred': {
      const { text, columnTest } = test.presentAndInferred
      if (!columnTest) {
        throw Error(
          `No column test for callback provided in ${JSON.stringify(test)}`,
        )
      }
      return (
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger>
              <Badge variant="default">
                <div className="flex items-center gap-1">
                  <InfoCircledIcon className="h-3 w-3" />
                  {text}
                  {removeColumnTest ? (
                    <CrossCircledIcon
                      onClick={() => {
                        removeColumnTest(columnTest)
                      }}
                      className="h-3 w-3 hover:cursor-pointer"
                    />
                  ) : null}
                </div>
              </Badge>
            </TooltipTrigger>
            <TooltipContent>
              <p>Inferred and in documnetstion</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      )
    }
    case 'notPresentButInferred': {
      const { text, columnTest } = test.notPresentButInferred
      if (!columnTest) {
        throw Error(
          `No column test for callback provided in ${JSON.stringify(test)}`,
        )
      }
      return (
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger>
              <Badge
                onClick={
                  addColumnTest ? () => addColumnTest(columnTest) : undefined
                }
                variant="outline"
              >
                {!addColumnTest ? (
                  <InfoCircledIcon className="h-3 w-3" />
                ) : null}
                <div className="flex items-center gap-1">
                  {addColumnTest ? (
                    <PlusCircledIcon className="h-3 w-3" />
                  ) : null}
                  {text}
                </div>
              </Badge>
            </TooltipTrigger>
            <TooltipContent>
              {addColumnTest ? (
                <p>Add inferred test to model</p>
              ) : (
                <p>Test is inferred</p>
              )}
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      )
    }
    default:
      throw new Error(`Unknown table test type: ${test}`)
  }
}

const TestWrapper: React.FC<{
  columnTitle: string
  tests: RowTest[]
  addColumnTest?: (columnTest: ColumnTest) => void
  removeColumnTest?: (columnTest: ColumnTest) => void
}> = ({ tests, columnTitle, addColumnTest, removeColumnTest }) => (
  <div className="flex items-center">
    <div className="flex flex-wrap gap-1.5" key={JSON.stringify(tests)}>
      {tests.map((tableTest) => (
        <TestBadge
          key={`${columnTitle}-test-${tableTest.test?.$case}`}
          rowTest={tableTest}
          addColumnTest={addColumnTest}
          removeColumnTest={removeColumnTest}
        />
      ))}
      {addColumnTest ? <AddTestDialog addColumnTest={addColumnTest} /> : null}
    </div>
  </div>
)

const DescriptionWrapper: React.FC<{
  description?: RowDescription
  addDescription?: (description: string) => void
}> = ({ description, addDescription }) => {
  const d = description?.description
  if (!d) {
    throw Error(`No description provided in ${JSON.stringify(description)}`)
  }
  switch (d.$case) {
    case 'present': {
      return (
        <DescriptionCell
          addDescription={addDescription}
          descriptionText={d.present}
        />
      )
    }
    case 'presentAndInferredIdentical': {
      return (
        <DescriptionCell
          addDescription={addDescription}
          descriptionText={d.presentAndInferredIdentical}
        />
      )
    }
    case 'presentWithDifferentInference': {
      return (
        <DescriptionCell
          addDescription={addDescription}
          descriptionText={d.presentWithDifferentInference.present}
        />
      )
    }
    case 'inferred': {
      return (
        <InferredDescription
          title={d.inferred}
          addDescription={
            addDescription ? () => addDescription(d.inferred) : undefined
          }
        />
      )
    }
    case 'notPresent': {
      return <DescriptionCell addDescription={addDescription} />
    }
    default: {
      throw new Error(`Unknown description type: ${d}`)
    }
  }
}

const DescriptionCell: React.FC<{
  descriptionText?: string
  addDescription?: (description: string) => void
}> = ({ descriptionText, addDescription }) => {
  const [editDescriptionMode, setEditDescriptionMode] = useState<boolean>(false)
  const [stagedDescriptionText, setStagedDescriptionText] = useState<
    string | undefined
  >(descriptionText)

  return (
    <div>
      {editDescriptionMode ? (
        <div className="flex items-center gap-1">
          <Input
            id="columnDescriptionInput"
            value={stagedDescriptionText}
            onInput={(e) => {
              setStagedDescriptionText(
                (e.target as HTMLTextAreaElement).value as string,
              )
            }}
            placeholder="Enter a description for this column."
          />
          <div className="flex flex-col">
            <CrossCircledIcon
              className="h-4 w-4 hover:cursor-pointer"
              onClick={() => {
                setStagedDescriptionText(descriptionText)
                setEditDescriptionMode(false)
              }}
            />

            <CheckCircledIcon
              className="h-4 w-4 hover:cursor-pointer"
              onClick={() => {
                if (addDescription) {
                  addDescription(stagedDescriptionText || '')
                }
                setEditDescriptionMode(false)
              }}
            />
          </div>
        </div>
      ) : (
        <>
          {descriptionText ? (
            <p>
              {descriptionText}
              {addDescription ? (
                <Button variant="link" size="sm">
                  <Pencil2Icon
                    className="h-4 w-4"
                    onClick={() => {
                      setEditDescriptionMode(true)
                    }}
                  />
                </Button>
              ) : null}
            </p>
          ) : (
            <div className="flex items-center justify-center">
              <Badge
                variant="destructive"
                className="flex items-center gap-1 hover:cursor-pointer"
                onClick={
                  addDescription
                    ? () => setEditDescriptionMode(true)
                    : undefined
                }
              >
                {addDescription ? <Pencil2Icon className="h-4 w-4" /> : null}
                No Description
              </Badge>
            </div>
          )}
        </>
      )}
    </div>
  )
}

const InferredDescription: React.FC<{
  title: string
  addDescription?: () => void
}> = ({ title, addDescription }) => (
  <div className="flex items-center justify-center">
    <p className="flex-grow">{title}️</p>
    {addDescription ? (
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Badge onClick={addDescription} variant="secondary">
              <div className="flex items-center gap-1">
                <PlusCircledIcon className="h-3 w-3" />
                Inferred
              </div>
            </Badge>
          </TooltipTrigger>
          <TooltipContent>
            <p>Add inferred description to model</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    ) : (
      <Badge variant="secondary">Inferred</Badge>
    )}
  </div>
)

const RemoveButton: React.FC<{ onClick: () => void }> = ({ onClick }) => (
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger>
        <Badge onClick={onClick} variant="secondary">
          <div className="flex items-center gap-1">
            <MinusCircledIcon className="h-3 w-3" />
            Remove
          </div>
        </Badge>
      </TooltipTrigger>
      <TooltipContent>
        <p>Remove column from list</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
)
