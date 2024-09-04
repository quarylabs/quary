import { useEffect, useRef } from 'react'
import perspective from '@finos/perspective'
import '@finos/perspective-viewer'
import '@finos/perspective-viewer-datagrid'
import '@finos/perspective-viewer-d3fc'
import '@finos/perspective-viewer/dist/css/pro.css'
// eslint-disable-next-line no-duplicate-imports
import type { HTMLPerspectiveViewerElement } from '@finos/perspective-viewer'
import { QueryResult } from '@quary/proto/quary/service/v1/query_result'
import { dropNullValuesInJSONLike, JSONValue } from '@shared/jsonValue'

interface Props {
  title?: string
  results: QueryResult
  updateConfigListener?: (chartDefinition: JSONValue) => void
  openWithSettings?: boolean
  existingSettings?: JSONValue
}

export const Perspective: React.FC<Props> = ({
  results,
  updateConfigListener,
  openWithSettings,
  title,
  existingSettings,
}) => {
  const viewerRef = useRef<HTMLPerspectiveViewerElement | null>(null)

  useEffect(() => {
    const loadPerspective = async () => {
      const worker = await perspective.worker()
      const table = await worker.table(
        results.columns.reduce(
          (acc, row) => {
            acc[row.name] = row.values
            return acc
          },
          {} as Record<string, string[]>,
        ),
      )

      const el = document.querySelector('perspective-viewer')

      if (el) {
        await el.load(table)
        if (existingSettings) {
          el.restore({
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            ...existingSettings,
            title,
            settings: openWithSettings ?? false,
          })
        } else {
          el.restore({
            title,
            settings: openWithSettings ?? false,
          })
        }
      }

      if (el && updateConfigListener) {
        el.addEventListener('perspective-config-update', async () => {
          const config = await el.save('json')
          const betterConfig = dropNullValuesInJSONLike(config)
          updateConfigListener(betterConfig)
        })
      }
    }

    loadPerspective()
  }, [
    results.columns,
    existingSettings,
    openWithSettings,
    title,
    updateConfigListener,
  ])

  return (
    <div
      style={{
        width: '100%',
        height: '100%',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      }}
    >
      <perspective-viewer
        style={{ width: '100%', height: '500px' }}
        ref={viewerRef}
      ></perspective-viewer>
    </div>
  )
}
