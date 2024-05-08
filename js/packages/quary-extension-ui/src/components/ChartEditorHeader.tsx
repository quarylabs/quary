import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import * as z from 'zod'
import { PencilSquareIcon, PlayIcon } from '@heroicons/react/20/solid'
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
}

export const ChartEditorHeader: React.FC<Props> = ({
  data,
  allAssets,
  disabled,
  onChangeSource: onChangeSourceProp,
  onClickRunQuery,
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
        onChangeSource={changeState}
      />
      <Button
        disabled={disabled}
        size="icon"
        onClick={() => onClickRunQuery(mapFormToChartFileSource(values))}
      >
        <PlayIcon className="h-4 w-4" />
      </Button>
      <Button size="icon" onClick={onClickEdit}>
        <PencilSquareIcon className="h-4 w-4" />
      </Button>
    </div>
  )
}

interface SubFormProps {
  values: FormValues
  disabled: boolean
  onChangeSource: (source: ChartFile['source']) => void
  allAssets: string[]
}

const SubForm: React.FC<SubFormProps> = ({
  values,
  disabled,
  onChangeSource,
  allAssets,
}) => {
  switch (values.type) {
    // TODO Implement other cases
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
        <div className="flex-1">
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
