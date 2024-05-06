import {
  columnsValuesToQueryResult,
  queryResultToColumnsValues,
} from '../src/shared'

describe('columnsValuesToQueryResult', () => {
  it('should transpose the values', () => {
    const columns = ['a', 'b'].map((column) => ({ column }))
    const values = [
      ['1', '2'],
      ['3', '4'],
    ]
    const values_2 = [
      ['1', '2'],
      ['3', '4'],
    ]

    const queryResult = columnsValuesToQueryResult({ columns, values })
    expect(queryResult.columns).toEqual([
      { name: 'a', values: ['1', '3'] },
      { name: 'b', values: ['2', '4'] },
    ])
    expect(values).toEqual(values_2)
  })

  it('should transpose the values in non square matrix', () => {
    const columns = ['a', 'b', 'c', 'd'].map((column) => ({ column }))
    const values = [
      ['1', '2', '3', '4'],
      ['3', '4', '5', '6'],
    ]
    const values_2 = [
      ['1', '2', '3', '4'],
      ['3', '4', '5', '6'],
    ]

    const queryResult = columnsValuesToQueryResult({ columns, values })
    expect(queryResult.columns).toEqual([
      { name: 'a', values: ['1', '3'] },
      { name: 'b', values: ['2', '4'] },
      { name: 'c', values: ['3', '5'] },
      { name: 'd', values: ['4', '6'] },
    ])
    expect(values).toEqual(values_2)
  })

  it('should transpose empty values', () => {
    const columns = ['a', 'b', 'c', 'd'].map((column) => ({ column }))
    const values: string[][] = []
    const values_2: string[][] = []

    const queryResult = columnsValuesToQueryResult({ columns, values })
    expect(queryResult.columns).toEqual([
      { name: 'a', values: [] },
      { name: 'b', values: [] },
      { name: 'c', values: [] },
      { name: 'd', values: [] },
    ])
    expect(values).toEqual(values_2)
  })

  it('should transpose nothing', () => {
    const columns: { column: string }[] = []
    const values: string[][] = []
    const values_2: string[][] = []

    const queryResult = columnsValuesToQueryResult({ columns, values })
    expect(queryResult.columns).toEqual([])
    expect(values).toEqual(values_2)
  })
})

describe('queryResultToColumnsValues', () => {
  it('should transpose the values', () => {
    const queryResult = {
      columns: [
        { name: 'a', values: ['1', '3'] },
        { name: 'b', values: ['2', '4'] },
      ],
    }
    const queryResult2 = {
      columns: [
        { name: 'a', values: ['1', '3'] },
        { name: 'b', values: ['2', '4'] },
      ],
    }

    const { columns, values } = queryResultToColumnsValues(queryResult)
    expect(columns).toEqual(['a', 'b'].map((column) => ({ column })))
    expect(values).toEqual([
      ['1', '2'],
      ['3', '4'],
    ])
    expect(queryResult).toEqual(queryResult2)
  })

  it('should transpose empty values', () => {
    const queryResult = {
      columns: [
        { name: 'a', values: [] },
        { name: 'b', values: [] },
      ],
    }
    const queryResult2 = {
      columns: [
        { name: 'a', values: [] },
        { name: 'b', values: [] },
      ],
    }

    const { columns, values } = queryResultToColumnsValues(queryResult)
    expect(columns).toEqual(['a', 'b'].map((column) => ({ column })))
    expect(values).toEqual([])
    expect(queryResult).toEqual(queryResult2)
  })

  it('should transpose non square values', () => {
    const queryResult = {
      columns: [
        { name: 'a', values: ['1', '4'] },
        { name: 'b', values: ['2', '5'] },
        { name: 'c', values: ['3', '6'] },
      ],
    }
    const queryResult2 = {
      columns: [
        { name: 'a', values: ['1', '4'] },
        { name: 'b', values: ['2', '5'] },
        { name: 'c', values: ['3', '6'] },
      ],
    }

    const { columns, values } = queryResultToColumnsValues(queryResult)
    expect(columns).toEqual(['a', 'b', 'c'].map((column) => ({ column })))
    expect(values).toEqual([
      ['1', '2', '3'],
      ['4', '5', '6'],
    ])
    expect(queryResult).toEqual(queryResult2)
  })
})
