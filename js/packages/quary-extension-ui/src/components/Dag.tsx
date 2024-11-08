/* eslint-disable react-refresh/only-export-components */
import { useMemo } from 'react'
import {
  Controls,
  MarkerType,
  ReactFlow,
  Background,
  BackgroundVariant,
} from 'reactflow'
import dagre from '@dagrejs/dagre'
import 'reactflow/dist/style.css'
import './Dag.css'
import { Dag as DagType } from '@shared/globalViewState'
import { Err, isErr, Ok, ResultE } from '@shared/result'
import { cn } from '@ui/lib/utils'
import { Warning } from './Warning'
import { DagNode, DagNodeDataType } from './DagNode'

const NODE_SPACING = 200
const LAYER_SPACING = 20

interface Props {
  dag: DagType
  onNodeDoubleClick?: (node: DagNodeDataType) => void
  className?: string
  optionalDagNodeOverride?: React.FC<React.ComponentProps<typeof DagNode>>
  overrideFlowEdgeStroke?: string
  isFullProjectDag?: boolean
}

export type DagLocalType = {
  nodes: {
    modelName: string
    presentCacheView?: boolean
    modelOrSeedOrSource?: number
    filePath?: string
  }[]
  edges: { from: string; to: string }[]
}

const transformProjectDag = ({ dag, models }: DagType): DagLocalType => ({
  nodes: dag.nodes.map((node) => {
    // TODO: error handling for when node.id is not found or return additional metadata in rust
    const matchingModel = models.find((model) => model.name === node.id)
    return {
      modelName: node.id,
      presentCacheView: node.isCached,
      modelOrSeedOrSource: matchingModel?.assetType, // TODO: make non-optional
      filePath: matchingModel?.filePath, // TODO: make non-optional
    }
  }),
  edges: dag.edges,
})

export const Dag: React.FC<Props> = ({
  dag: untransformedDag,
  onNodeDoubleClick,
  className,
  optionalDagNodeOverride,
  overrideFlowEdgeStroke,
  isFullProjectDag = false,
}) => {
  const dag = useMemo(
    () => transformProjectDag(untransformedDag),
    [untransformedDag],
  )

  const { flowNodes, flowEdges, error } = useMemo(() => {
    const { nodes, edges } = mapDagTypeToDagModel(dag)

    const targetNodeResult = isFullProjectDag
      ? findFullProjectTargetNodes(nodes, edges)
      : findTargetNode(nodes, edges)

    if (isErr(targetNodeResult)) {
      return { error: targetNodeResult.error }
    }

    const organisedNodesResult = applyLayout(
      nodes,
      edges,
      targetNodeResult.value.id,
    )
    if (isErr(organisedNodesResult)) {
      return { error: organisedNodesResult.error }
    }

    const flowEdges = edges.map(({ source, target }) => ({
      id: `${source}-${target}`,
      source,
      target,
      animated: true,
      style: {
        stroke: overrideFlowEdgeStroke ?? 'var(--vscode-editor-foreground)',
      },
      markerEnd: {
        type: MarkerType.ArrowClosed,
      },
    }))

    return { flowNodes: organisedNodesResult.value, flowEdges, error: null }
  }, [dag, overrideFlowEdgeStroke, isFullProjectDag])

  if (error) {
    return <Warning title={error} />
  }

  const nodeTypes = optionalDagNodeOverride
    ? { quaryNode: optionalDagNodeOverride }
    : { quaryNode: DagNode }

  return (
    <div className={cn('h-64', className)}>
      <ReactFlow
        defaultNodes={flowNodes}
        defaultEdges={flowEdges}
        fitView
        nodeTypes={nodeTypes}
        onNodeDoubleClick={(_, node) => {
          const nodeData = node.data as DagNodeDataType
          onNodeDoubleClick?.(nodeData)
        }}
      >
        <Controls
          position="top-right"
          showZoom={false}
          showFitView={true}
          showInteractive={false}
        />
        <Background
          color="var(--vscode-statusBarItem-prominentForeground)"
          variant={BackgroundVariant.Dots}
          gap={30}
          size={1}
        />
      </ReactFlow>
    </div>
  )
}

export const applyLayout = (
  nodes: DagNode[],
  edges: DagEdge[],
  targetNodeId: string,
): ResultE<
  {
    id: string
    type: string
    data: DagNodeDataType
    position: Position
  }[],
  string
> => {
  try {
    const dagreGraph = new dagre.graphlib.Graph()
    dagreGraph.setDefaultEdgeLabel(() => ({}))
    dagreGraph.setGraph({
      rankdir: 'LR',
      align: 'UL',
      marginx: NODE_SPACING,
      marginy: LAYER_SPACING,
    })

    nodes.forEach((node) => {
      const nodeWidth = Math.max(NODE_SPACING, node.label.length * 10)
      const nodeHeight = LAYER_SPACING
      dagreGraph.setNode(node.id, {
        width: nodeWidth,
        height: nodeHeight,
      })
    })

    edges.forEach((edge) => {
      dagreGraph.setEdge(edge.source, edge.target)
    })

    dagre.layout(dagreGraph)

    const flowNodes = nodes.map(
      ({ id, label, backGroundLabel, type, filePath }) => ({
        id,
        type: 'quaryNode',
        data: {
          highlight: id === targetNodeId,
          label,
          backGroundLabel,
          type,
          filePath,
          columns: [],
        },
        position: {
          x: dagreGraph.node(id).x,
          y: dagreGraph.node(id).y,
        },
      }),
    )
    return Ok(flowNodes)
  } catch (e) {
    if (e instanceof Error) {
      return Err(e.message)
    }
    return Err('Unknown error')
  }
}

export const findFullProjectTargetNodes = (
  nodes: DagNode[],
  edges: DagEdge[],
): ResultE<DagNode, string> => {
  if (nodes.length === 0) {
    return Err('Expected at least one node')
  }
  const targets = edges
    .map(({ target }) => target)
    .reduce((acc, el) => acc.add(el), new Set<string>())
  const sources = edges
    .map(({ source }) => source)
    .reduce((acc, el) => acc.add(el), new Set<string>())

  const unconnectedNodes = nodes.filter(
    (n) => !targets.has(n.id) && !sources.has(n.id),
  )

  const nodesThatAreTargetsAndNotSources = nodes.filter(
    (n) => targets.has(n.id) && !sources.has(n.id),
  )

  const targetNode = nodesThatAreTargetsAndNotSources[0] || unconnectedNodes[0]

  if (!targetNode) {
    return Err('No suitable target node found')
  }

  return Ok(targetNode)
}

export const findTargetNode = (
  nodes: DagNode[],
  edges: DagEdge[],
): ResultE<DagNode, string> => {
  if (nodes.length === 0) {
    return Err('Expected at least one node')
  }
  if (nodes.length === 1 && edges.length === 0) {
    return Ok(nodes[0])
  }
  const targets = edges
    .map(({ target }) => target)
    .reduce((acc, el) => acc.add(el), new Set<string>())
  const sources = edges
    .map(({ source }) => source)
    .reduce((acc, el) => acc.add(el), new Set<string>())

  const nodesThatAreTargetsAndNotSources = nodes.filter(
    (n) => targets.has(n.id) && !sources.has(n.id),
  )
  if (nodesThatAreTargetsAndNotSources.length !== 1) {
    return Err(
      `Expected one node that is just a target, got ${
        nodesThatAreTargetsAndNotSources.length
      }, nodes ${nodesThatAreTargetsAndNotSources.map((n) => n.id)}`,
    )
  }
  // Assuming there's only one root node, return the first one found
  return Ok(nodesThatAreTargetsAndNotSources[0])
}

const mapDagTypeToDagModel = (dag: DagLocalType): DagModel => {
  const { nodes, edges } = dag
  return {
    nodes: nodes.map(
      ({ modelName, presentCacheView, modelOrSeedOrSource, filePath }) => ({
        id: modelName,
        label: modelName,
        backGroundLabel: presentCacheView!,
        type: modelOrSeedOrSource,
        filePath,
      }),
    ),
    edges: edges.map(({ from, to }) => ({ source: from, target: to })),
  }
}

export type Position = {
  x: number
  y: number
}

export interface DagModel {
  nodes: DagNode[]
  edges: DagEdge[]
}

export type DagEdge = {
  source: string
  target: string
}

export interface DagNode {
  id: string
  label: string
  backGroundLabel: boolean
  type?: number
  filePath?: string
  columns: string[]
}
