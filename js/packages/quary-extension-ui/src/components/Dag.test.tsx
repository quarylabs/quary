// eslint-disable-next-line import/no-extraneous-dependencies
import { expect, it, describe } from 'vitest'
import { isErr } from '@shared/result'
import { DagEdge, DagNode, findTargetNode, applyLayout } from '@/components/Dag'

describe('findTargetNode', () => {
  it('should solve a simple example with one node', () => {
    const nodes: DagNode[] = [
      {
        id: '1',
        label: '1',
      },
    ]
    const edges: DagEdge[] = []

    const result = findTargetNode(nodes, edges)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual(nodes[0])
  })

  it('should solve a simple example with two nodes', () => {
    const nodes: DagNode[] = [
      {
        id: '1',
        label: '1',
      },
      {
        id: '2',
        label: '2',
      },
    ]
    const edges: DagEdge[] = [
      {
        source: nodes[0].id,
        target: nodes[1].id,
      },
    ]

    const result = findTargetNode(nodes, edges)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual(nodes[1])
  })

  it('should solve a simple example with three nodes', () => {
    const nodes: DagNode[] = [
      {
        id: '1',
        label: '1',
      },
      {
        id: '2',
        label: '2',
      },
      {
        id: '3',
        label: '3',
      },
    ]
    const edges: DagEdge[] = [
      {
        source: nodes[0].id,
        target: nodes[2].id,
      },
      {
        source: nodes[1].id,
        target: nodes[2].id,
      },
    ]

    const result = findTargetNode(nodes, edges)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual(nodes[2])
  })

  it('should resolve a more complex tree', () => {
    const nodes: DagNode[] = [
      { id: 'a', label: 'a' },
      { id: 'b', label: 'b' },
      { id: 'c', label: 'c' },
      { id: 'd', label: 'd' },
    ]
    const edges: DagEdge[] = [
      { source: 'a', target: 'b' },
      { source: 'c', target: 'b' },
      { source: 'b', target: 'd' },
    ]

    const result = findTargetNode(nodes, edges)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual({ id: 'd', label: 'd' })
  })
})

describe('dagreGraphLayout', () => {
  it('should solve a simple example with one node', () => {
    const nodes: DagNode[] = [
      {
        id: '1',
        label: '1',
      },
    ]
    const edges: DagEdge[] = []

    const target = findTargetNode(nodes, edges)
    if (isErr(target)) {
      throw new Error('target not found')
    }

    const result = applyLayout(nodes, edges, target.value)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual([
      {
        ...result.value[0],
        id: '1',
      },
    ])
  })

  it('should solve a simple example with two nodes', () => {
    const nodes: DagNode[] = [
      {
        id: '1',
        label: '1',
      },
      {
        id: '2',
        label: '2',
      },
    ]
    const edges: DagEdge[] = [
      {
        source: nodes[0].id,
        target: nodes[1].id,
      },
    ]

    const target = findTargetNode(nodes, edges)
    if (isErr(target)) {
      throw new Error('target not found')
    }

    const result = applyLayout(nodes, edges, target.value)
    if (isErr(result)) {
      throw new Error('target not found')
    }

    expect(result.value).toEqual([
      {
        ...result.value[0],
        id: '1',
      },
      {
        ...result.value[1],
        id: '2',
      },
    ])
  })
})
