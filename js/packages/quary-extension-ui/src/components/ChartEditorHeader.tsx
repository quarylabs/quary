import React, { useState } from 'react'
import { PencilSquareIcon, PlayIcon, PlusIcon } from '@heroicons/react/20/solid'
import { useCallBackFrontEnd } from '@shared/callBacks.ts'
import { z } from 'zod'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import { vscode } from '@ui/utils/VSCodeAPIWrapper.ts'

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from '@ui/components/ui/select'
import {
  TooltipProvider,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@ui/components/ui/tooltip'
import { Button } from '@ui/components/ui/button'
import { Input } from '@ui/components/ui/input'

interface Props {
  chartFileSource: ChartFile['source']
  assets: string[]
  disabled: boolean
}

export const ChartEditorHeader: React.FC<Props> = ({
  assets,
  disabled,
  chartFileSource,
}) => {
  const {
    chartViewMakeSourceEdit,
    chartViewCreateModel,
    chartViewRunQuery,
    chartViewOpenTextEditor,
  } = useCallBackFrontEnd(
    [
      'chartViewMakeSourceEdit',
      'chartViewCreateModel',
      'chartViewRunQuery',
      'chartViewOpenTextEditor',
    ],
    vscode.postMessage,
  )

  // intermediary source state for the header component
  const [stagedChartFileSource, setStagedChartFileSource] =
    useState<ChartFile['source']>(chartFileSource)

  const onChangeSource = (source: ChartFile['source']) => {
    // update the staged source
    setStagedChartFileSource(source)
    // emit a change to the file (makeEditSource)
    chartViewMakeSourceEdit(source)
  }

  const formSourceValue = mapChartFileSourceToForm(stagedChartFileSource)

  return (
    <div className="flex flex-row items-center gap-1">
      <Select
        value={formSourceValue.type}
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
                  name: assets[0],
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
        disabled={disabled}
        formSourceValue={formSourceValue}
        assets={assets}
        onChangeSource={onChangeSource}
        chartViewCreateModel={chartViewCreateModel}
      />
      <TooltipButton
        icon={<PlayIcon className="h-4 w-4" />}
        onClick={() => {
          chartViewRunQuery(null)
        }}
        disabled={
          disabled ||
          // disable the run button if no asset is selected
          (formSourceValue.type === 'reference' && !formSourceValue.reference)
        }
        tooltip="Run Query"
      />
      <TooltipButton
        icon={<PencilSquareIcon className="h-4 w-4" />}
        onClick={() => chartViewOpenTextEditor(null)}
        disabled={disabled}
        tooltip="Edit Yaml"
      />
    </div>
  )
}

interface SubFormProps {
  formSourceValue: FormSourceValue
  disabled: boolean
  onChangeSource: (source: ChartFile['source']) => void
  assets: string[]
  // create a model from the sql should open file
  chartViewCreateModel: (sql: string) => void
}

const SubForm: React.FC<SubFormProps> = ({
  formSourceValue,
  disabled,
  onChangeSource,
  assets,
  chartViewCreateModel,
}) => {
  switch (formSourceValue.type) {
    case 'rawSql':
      return (
        <Input
          className="flex-1"
          disabled={disabled}
          value={formSourceValue.rawSql}
          onChange={(e) =>
            onChangeSource({ $case: 'rawSql', rawSql: e.target.value })
          }
        />
      )
    case 'preTemplatedSql':
      return (
        <>
          <Input
            className="flex-1"
            disabled={disabled}
            value={formSourceValue.preTemplatedSql}
            onChange={(e) =>
              onChangeSource({
                $case: 'preTemplatedSql',
                preTemplatedSql: e.target.value,
              })
            }
          />
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger>
                <Button
                  size="icon"
                  onClick={() => {
                    chartViewCreateModel(formSourceValue.preTemplatedSql)
                  }}
                  disabled={disabled}
                >
                  <PlusIcon className="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Create Model</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </>
      )
    case 'reference':
      return (
        <div className="flex-1">
          <Select
            defaultValue={formSourceValue.reference}
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
                {assets.map((asset) => (
                  <SelectItem key={asset} value={asset}>
                    {asset}
                  </SelectItem>
                ))}
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      )
    default:
      return null
  }
}

const mapChartFileSourceToForm = (
  source: ChartFile['source'],
): FormSourceValue => {
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

interface TooltipButtonProps {
  icon: React.ReactNode
  onClick: () => void
  disabled: boolean
  tooltip: string
}

const TooltipButton: React.FC<TooltipButtonProps> = ({
  icon,
  onClick,
  disabled,
  tooltip,
}) => (
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger>
        <Button size="icon" onClick={onClick} disabled={disabled}>
          {icon}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{tooltip}</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
)

type FormSourceValue = z.infer<typeof FormSchema>
