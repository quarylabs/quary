import { ChartDocument } from './chartCustomEditorChartDocument'
import { ChartFile } from '@quary/proto/quary/service/v1/chart_file'
import * as vscode from 'vscode'
import { PreInitServices } from './services'

describe('ChartDocument', () => {
  let chartDocument: ChartDocument
  let onDidChangeMock: jest.Mock
  let services: PreInitServices

  beforeEach(async () => {
    const uri = vscode.Uri.parse('untitled:test.chart')
    const delegate = {
      getFileData: jest.fn().mockResolvedValue(new Uint8Array()),
    }
    services = {
      rust: {
        parse_chart_file: jest
          .fn()
          .mockReturnValue({ value: ChartFile.create({}) }),
        write_chart_file_to_yaml: jest
          .fn()
          .mockReturnValue({ value: new Uint8Array() }),
      },
    } as unknown as PreInitServices
    chartDocument = await ChartDocument.create(
      uri,
      undefined,
      delegate,
      services,
    )
    onDidChangeMock = jest.fn()
    chartDocument.onDidChange(onDidChangeMock)
  })

  test('chart without content change should not trigger change notification', () => {
    // Initialize the document with some data to simulate existing state
    const initialData = ChartFile.create({
      description: 'Test description',
      config: {},
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument['documentData'] = initialData

    // Apply an edit that does not alter the document's state
    chartDocument.makeEdit(initialData)

    expect(onDidChangeMock).not.toHaveBeenCalled()
    expect(chartDocument['_edits']).toEqual([initialData])
  })

  test('chart content change should trigger change notification', () => {
    // Initialize the document with initial data
    const initialData = ChartFile.create({
      description: 'Initial description',
      config: {},
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument['documentData'] = initialData

    // Apply an edit that changes the document's source
    const newEdit = ChartFile.create({
      description: 'Initial description',
      config: {},
      tags: [],
      source: {
        $case: 'reference',
        reference: { name: 'shift_hours' },
      },
    })
    chartDocument.makeEdit(newEdit)

    expect(onDidChangeMock).toHaveBeenCalledTimes(1)
    expect(chartDocument['_edits']).toEqual([initialData, newEdit])
    expect(chartDocument.documentData).toEqual(newEdit)
  })

  test('changes to config.settings should be ignored in edit detection', () => {
    // Initialize the document with some settings
    const initialData = ChartFile.create({
      description: 'Test description',
      config: {
        settings: false,
      },
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument['documentData'] = initialData

    // Apply an edit that changes only the settings
    const settingsEdit = ChartFile.create({
      description: 'Test description',
      config: {
        settings: true,
      },
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument.makeEdit(settingsEdit)

    expect(onDidChangeMock).not.toHaveBeenCalled()
    expect(chartDocument['_edits']).toEqual([initialData])
    expect(chartDocument.documentData).toEqual(initialData)
  })

  test('changes to config order should be ignored in edit detection', () => {
    // Initialize the document with some settings
    const initialData = ChartFile.create({
      description: 'Test description',
      config: {
        1: '1',
        2: '2',
        3: '3',
      },
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument['documentData'] = initialData

    // Apply an edit that changes only the settings
    const settingsEdit = ChartFile.create({
      description: 'Test description',
      config: {
        3: '3',
        2: '2',
        1: '1',
      },
      tags: [],
      source: {
        $case: 'rawSql',
        rawSql: 'SELECT * FROM test',
      },
    })
    chartDocument.makeEdit(settingsEdit)

    expect(onDidChangeMock).not.toHaveBeenCalled()
    expect(chartDocument['_edits']).toEqual([initialData])
    expect(chartDocument.documentData).toEqual(initialData)
  })
})
