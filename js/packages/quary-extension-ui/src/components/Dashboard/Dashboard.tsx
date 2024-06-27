import { Layout, Responsive, WidthProvider } from 'react-grid-layout'
import { useEffect, useMemo, useState } from 'react'
import './react-grid-layout.css'
import { DashboardEditorDataItem } from '@shared/globalViewState'
import { Perspective } from '@ui/components/Perspective'
import { ProgressRing } from '@ui/components/ProgressRing'
import { Dashboard as DashboardType } from '@quary/proto/quary/service/v1/dashboard'

interface Props {
  // className: string
  // cols: Record<string, number>
  // onLayoutChange: (currentLayout: Layout[], allLayouts: Layouts) => void
  // rowHeight: number
  dashboardItems: DashboardEditorDataItem[]
  dashboard: DashboardType
}

export type ItemWithInfo = {
  item: DashboardEditorDataItem
  layout: Layout
}

interface State {
  // TODO Need to constrain this string
  currentBreakpoint: string
  compactType: CompactType
  mounted: boolean
  resizeHandles: string[]
  layouts: {
    lg: Array<ItemWithInfo>
  }
}

export type CompactType = 'horizontal' | 'vertical'

export const Dashboard: React.FC<Props> = (props) => {
  const [state, setState] = useState<State>({
    currentBreakpoint: 'lg',
    compactType: 'vertical',
    mounted: false,
    resizeHandles: ['se'],
    layouts: {
      lg: [],
    },
  })

  const cols = { lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 }

  // Use useMemo to derive layouts from props
  const layouts = useMemo(
    () => ({
      lg: generateLayoutFromDashboardItems(props.dashboardItems),
    }),
    [props.dashboardItems],
  )

  // Use useEffect to update the state when props change
  useEffect(() => {
    setState((prevState) => ({
      ...prevState,
      layouts,
      mounted: true,
    }))
  }, [layouts])

  function generateChildren() {
    return layouts.lg.map((l, i) => {
      switch (l.item.result.type) {
        case 'loading':
          return (
            <div key={`${i}_loading`}>
              <div className="flex justify-center pt-10">
                <ProgressRing className="h-20 w-20" />
              </div>
            </div>
          )
        case 'error':
          return (
            <div key={`${i}_error`}>
              <span className="text">
                {JSON.stringify(l.item.result.error)}
              </span>
            </div>
          )
        case 'success':
          return (
            <div key={`${i}_success`}>
              <Perspective
                results={l.item.result.queryResult}
                existingSettings={l.item.item.chart?.config}
                openWithSettings={false}
              />
            </div>
          )
        case 'not loaded':
          throw new Error('Not loaded should never happen')
        default:
          throw new Error('Unknown type')
      }
    })
  }

  const onBreakpointChange = (breakpoint: string) => {
    setState((prevState) => ({
      ...prevState,
      currentBreakpoint: breakpoint,
    }))
  }

  return (
    <div>
      {props.dashboard.title ? <h1>{props.dashboard.title}</h1> : null}
      {props.dashboard.description ? (
        <div>{props.dashboard.description}</div>
      ) : null}
      <ResponsiveReactGridLayout
        layouts={{
          lg: state.layouts.lg.map((l) => l.layout),
        }}
        onBreakpointChange={onBreakpointChange}
        measureBeforeMount={false}
        useCSSTransforms={state.mounted}
        compactType={state.compactType}
        preventCollision={!state.compactType}
        cols={cols}
      >
        {generateChildren()}
      </ResponsiveReactGridLayout>
    </div>
  )
}

const ResponsiveReactGridLayout = WidthProvider(Responsive)

function generateLayoutFromDashboardItems(
  dashboardItems: DashboardEditorDataItem[],
): Array<ItemWithInfo> {
  return dashboardItems.map((item, i) => {
    const innerItem = item.item.item
    if (!innerItem) {
      throw new Error('Item is missing')
    }
    return {
      item,
      layout: {
        x: innerItem.topLeftX,
        y: innerItem.topLeftY,
        w: innerItem.width,
        h: innerItem.height,
        i: i.toString(),
        resizeHandles: ['se'],
      },
    }
  })
}
