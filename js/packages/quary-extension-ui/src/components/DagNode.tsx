import React, { useState } from 'react'
import { Handle, Position } from 'reactflow'
import { twMerge } from 'tailwind-merge'
import {
  CircleStackIcon as DatabaseIcon,
  CubeIcon as ModelIcon,
  QuestionMarkCircleIcon as UnknownIcon,
  TableCellsIcon as SeedIcon,
  CameraIcon as SnapshotIcon,
} from '@heroicons/react/20/solid'
import { ListAssetsResponse_Asset_AssetType } from '@quary/proto/quary/service/v1/wasm_rust_rpc_calls'

export type DagNodeDataType = {
  label: string
  highlight: boolean
  backGroundLabel?: boolean
  type?: number
  filePath?: string
  columns: string[]
}

interface Props {
  data: DagNodeDataType
  turnOffHandles?: boolean
}

export const DagNode: React.FC<React.PropsWithChildren<Props>> = ({
  data,
  turnOffHandles,
}) => {
  const [showColumns, setShowColumns] = useState<boolean>(false)

  const nodeHighlightStyle = data.highlight
    ? 'bg-vscode-editorInfo-foreground text-white'
    : 'bg-white text-gray-800'

  return (
    <div className="rounded-lg shadow-2xl">
      {turnOffHandles ? null : (
        <Handle
          className="invisible"
          type="target"
          position={Position.Left}
          isConnectable={false}
        />
      )}
      {turnOffHandles ? null : (
        <Handle
          className="invisible"
          type="source"
          position={Position.Right}
          isConnectable={false}
        />
      )}
      <div
        className={twMerge(
          'flex min-w-fit items-center justify-center overflow-hidden rounded-lg',
          nodeHighlightStyle,
        )}
        style={{ position: 'relative' }}
      >
        <DagNodeTypeLabel type={data.type || -1} />
        {data.backGroundLabel && (
          <div className="bg-vscode-editorInfo-foreground text-vscode-button-foreground absolute right-0 top-0 rounded-bl-lg p-0.5 text-xs">
            cached
          </div>
        )}

        <div className="min-w-0 flex-1 px-3 py-2">
          <div className="text-ellipsis whitespace-nowrap">{data.label}</div>
        </div>
        {turnOffHandles ? null : (
          <Handle
            className="invisible"
            type="target"
            position={Position.Left}
            isConnectable={false}
          />
        )}
        {turnOffHandles ? null : (
          <Handle
            className="invisible"
            type="source"
            position={Position.Right}
            isConnectable={false}
          />
        )}
      </div>
      {data.columns.length > 0 ? (
        <>
          <div onClick={() => setShowColumns(!showColumns)}>
            {showColumns ? (
              <div className="pl-2">Hide</div>
            ) : (
              <div className="pl-10">Show</div>
            )}
          </div>
          {showColumns ? (
            <ul className="list-none space-y-2">
              {data.columns.map((column, index) => (
                <li key={index} className="border-b py-1 text-gray-700">
                  {column}
                </li>
              ))}
            </ul>
          ) : null}
        </>
      ) : null}
    </div>
  )
}

const DagNodeTypeLabel = ({ type }: { type: number }) => {
  const protoType = ListAssetsResponse_Asset_AssetType

  const componentTypes: {
    [key: number]: {
      component: React.FC
      label: string
      style: string
    }
  } = {
    [protoType.ASSET_TYPE_SEED]: {
      component: SeedIcon,
      label: 'SEED',
      style: 'bg-vscode-terminal-ansiGreen',
    },
    [protoType.ASSET_TYPE_SOURCE]: {
      component: DatabaseIcon,
      label: 'SRC',
      style: 'bg-vscode-terminal-ansiBrightBlack',
    },
    [protoType.ASSET_TYPE_SNAPSHOT]: {
      component: SnapshotIcon,
      label: 'SNP',
      style: 'bg-vscode-editorInfo-foreground',
    },
    [protoType.ASSET_TYPE_MODEL]: {
      component: ModelIcon,
      label: 'MDL',
      style: 'bg-vscode-editorInfo-foreground',
    },
  }

  const {
    component: IconComponent = UnknownIcon,
    label = '',
    style = 'bg-vscode-editorInfo-foreground',
  } = componentTypes[type] || {}

  const labelClasses = twMerge(
    'text-vscode-button-foreground  px-3 py-3 text-xs flex items-center justify-center flex-col rounded-full',
    style,
  )

  return (
    <div className={labelClasses}>
      <IconComponent className="text-vscode-button-foreground h-5 w-5" />
      {label}
      <div></div>
    </div>
  )
}
