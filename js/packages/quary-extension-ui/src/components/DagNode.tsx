import React from 'react'
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
}

interface Props {
  data: DagNodeDataType
}

export const DagNode: React.FC<React.PropsWithChildren<Props>> = ({ data }) => {
  const nodeBaseStyle =
    'flex items-center justify-center shadow-lg rounded-lg overflow-hidden min-w-fit'

  const nodeHighlightStyle = data.highlight
    ? 'bg-vscode-editorInfo-foreground text-white'
    : 'bg-white text-gray-800'

  const nodeClasses = twMerge(nodeBaseStyle, nodeHighlightStyle)

  return (
    <>
      <Handle
        className="invisible"
        type="target"
        position={Position.Left}
        isConnectable={false}
      />
      <Handle
        className="invisible"
        type="source"
        position={Position.Right}
        isConnectable={false}
      />
      <div className={nodeClasses} style={{ position: 'relative' }}>
        <DagNodeTypeLabel type={data.type || -1} />
        {data.backGroundLabel && (
          <div className="bg-vscode-editorInfo-foreground text-vscode-button-foreground absolute right-0 top-0 rounded-bl-lg p-0.5 text-xs">
            cached
          </div>
        )}

        <div className="min-w-0 flex-1 px-3 py-2">
          <div className="overflow-hidden text-ellipsis whitespace-nowrap">
            {data.label}
          </div>
        </div>
        <Handle
          className="invisible"
          type="target"
          position={Position.Left}
          isConnectable={false}
        />
        <Handle
          className="invisible"
          type="source"
          position={Position.Right}
          isConnectable={false}
        />
      </div>
    </>
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
    'text-vscode-button-foreground px-2 py-3 text-xs flex items-center justify-center flex-col',
    style,
  )

  return (
    <div className={labelClasses}>
      <IconComponent className="text-vscode-button-foreground h-5 w-5" />
      {label}
    </div>
  )
}
