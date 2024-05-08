import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import * as z from 'zod'
import { PencilSquareIcon, PlayIcon, PlusIcon } from '@heroicons/react/20/solid'
import React, { useState } from 'react'
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from './ui/select'
import { Button } from './ui/button'
import {
  TooltipProvider,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from './ui/tooltip'
import { Input } from '@/components/ui/input.tsx'

interface Props {
  data?: ChartFile['source']
  onChangeSource: (source: ChartFile['source']) => void

  allAssets: string[]
  // disable the buttons when things are loading
  disabled: boolean
  onClickRunQuery: (source: ChartFile['source']) => void
  // open text editor
  onClickEdit: () => void
  // create a model from the sql should open file
  onClickCreateModel: (sql: string) => void
}

export const ChartEditorHeader: React.FC<Props> = ({
  data,
  allAssets,
  disabled,
  onChangeSource: onChangeSourceProp,
  onClickRunQuery,
  onClickCreateModel,
  onClickEdit,
}) => {
  const [state, changeState] = useState(data)
  const values = mapChartFileSourceToForm(state)
  const onChangeSource = (source: ChartFile['source']) => {
    changeState(source)
    onChangeSourceProp(source)
  }

  return (
    <div className="flex flex-row items-center gap-1">
      <Select
        defaultValue={values.type}
        disabled={disabled}
        onValueChange={(value) => {
          switch (value) {
            case 'rawSql':
              return onChangeSource({
                $case: 'rawSql',
                rawSql: '',
              })
            case 'preTemplatedSql':
              return onChangeSource({
                $case: 'preTemplatedSql',
                preTemplatedSql: '',
              })
            case 'reference':
              return onChangeSource({
                $case: 'reference',
                reference: {
                  name: allAssets[0],
                },
              })
            default:
              throw new Error(`Unknown type: ${value}`)
          }
        }}
      >
        <SelectTrigger className="w-[180px]">
          <SelectValue placeholder="Select a type" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Type</SelectLabel>
            <SelectItem value="rawSql">Raw SQL</SelectItem>
            <SelectItem value="preTemplatedSql">Templated SQL</SelectItem>
            <SelectItem value="reference">Asset</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
      <SubForm
        allAssets={allAssets}
        disabled={disabled}
        values={values}
        onChangeSource={onChangeSource}
        onClickCreateModel={onClickCreateModel}
      />
      <RunQueryButton
        disabled={disabled}
        onClick={() => onClickRunQuery(mapFormToChartFileSource(values))}
      />
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <Button size="icon" onClick={onClickEdit} disabled={disabled}>
              <PencilSquareIcon className="h-4 w-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Edit Yaml</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  )
}

interface SubFormProps {
  values: FormValues
  disabled: boolean
  onChangeSource: (source: ChartFile['source']) => void
  allAssets: string[]
  onClickCreateModel: (sql: string) => void
}

const SubForm: React.FC<SubFormProps> = ({
  values,
  disabled,
  onChangeSource,
  allAssets,
  onClickCreateModel,
}) => {
  switch (values.type) {
    case 'rawSql':
      return (
        <div className="flex-1">
          <Input
            disabled={disabled}
            value={values.rawSql}
            onChange={(e) => {
              onChangeSource({
                $case: 'rawSql' as const,
                rawSql: e.target.value,
              })
            }}
          />
        </div>
      )
    case 'preTemplatedSql':
      return (
        <div className="flex flex-1 flex-row items-center gap-1">
          <Input
            disabled={disabled}
            value={values.preTemplatedSql}
            onChange={(e) => {
              onChangeSource({
                $case: 'preTemplatedSql',
                preTemplatedSql: e.target.value,
              })
            }}
          />
          <CreateModelButton
            onClick={() => onClickCreateModel(values.preTemplatedSql)}
            disabled={disabled}
          />
        </div>
      )
    case 'reference':
      return (
        <div className="flex-1">
          <Select
            defaultValue={values.reference}
            disabled={disabled}
            onValueChange={(value) => {
              onChangeSource({
                $case: 'reference',
                reference: {
                  name: value,
                },
              })
            }}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select an asset" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectLabel>Asset</SelectLabel>
                {allAssets.map((asset) => (
                  <SelectItem key={asset} value={asset}>
                    {asset}
                  </SelectItem>
                ))}
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      )
  }
}

const mapChartFileSourceToForm = (source: ChartFile['source']): FormValues => {
  if (!source) {
    return {
      type: 'rawSql',
      rawSql: '',
    }
  }
  switch (source.$case) {
    case 'rawSql':
      return {
        type: 'rawSql',
        rawSql: source.rawSql,
      }
    case 'preTemplatedSql':
      return {
        type: 'preTemplatedSql',
        preTemplatedSql: source.preTemplatedSql,
      }
    case 'reference':
      return {
        type: 'reference',
        reference: source.reference.name,
      }
  }
}

const mapFormToChartFileSource = (values: FormValues): ChartFile['source'] => {
  switch (values.type) {
    case 'rawSql':
      return {
        $case: 'rawSql',
        rawSql: values.rawSql,
      }
    case 'preTemplatedSql':
      return {
        $case: 'preTemplatedSql',
        preTemplatedSql: values.preTemplatedSql,
      }
    case 'reference':
      return {
        $case: 'reference',
        reference: {
          name: values.reference,
        },
      }
  }
}

const FormSchema = z.union([
  z.object({
    type: z.literal('rawSql'),
    rawSql: z.string({}),
  }),
  z.object({
    type: z.literal('preTemplatedSql'),
    preTemplatedSql: z.string({}),
  }),
  z.object({
    type: z.literal('reference'),
    reference: z.string({}),
  }),
])

type FormValues = z.infer<typeof FormSchema>

const CreateModelButton = ({
  onClick,
  disabled,
}: {
  onClick: () => void
  disabled: boolean
}) => (
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger>
        <Button size="icon" onClick={onClick} disabled={disabled}>
          <PlusIcon className="h-4 w-4" />
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>Create Model</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
)

const RunQueryButton = ({
  onClick,
  disabled,
}: {
  onClick: () => void
  disabled: boolean
}) => (
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger>
        <Button size="icon" onClick={onClick} disabled={disabled}>
          <PlayIcon className="h-4 w-4" />
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>Run Query</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
)
