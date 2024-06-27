import { Layout, Layouts, Responsive, WidthProvider } from 'react-grid-layout'
import _ from 'lodash'
import { EventHandler, Component, SyntheticEvent } from 'react'
import './react-grid-layout.css'
import { DashboardEditorDataItem } from '@shared/globalViewState'
import { Perspective } from '@ui/components/Perspective'
import { ProgressRing } from '@ui/components/ProgressRing'

interface Props {
  className: string
  cols: Record<string, number>
  onLayoutChange: (currentLayout: Layout[], allLayouts: Layouts) => void
  rowHeight: number
  dashboardItems: DashboardEditorDataItem[]
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

const availableHandles = ['s', 'w', 'e', 'n', 'sw', 'nw', 'se', 'ne']

export default class Dashboard extends Component<Props, State> {
  static defaultProps: Props = {
    className: 'layout',
    rowHeight: 30,
    onLayoutChange() {},
    cols: { lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 },
    dashboardItems: [],
  }

  state: State = {
    currentBreakpoint: 'lg',
    compactType: 'vertical',
    mounted: false,
    resizeHandles: ['se'],
    layouts: {
      lg: generateLayoutFromDashboardItems(this.props.dashboardItems),
    },
  }

  componentDidMount() {
    this.setState({
      mounted: true,
    })
  }

  generateDOM(): React.ReactNode[] {
    return this.state.layouts.lg.map((l, i) => {
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

  onBreakpointChange: (breakpoint: string) => void = (breakpoint) => {
    this.setState({
      currentBreakpoint: breakpoint,
    })
  }

  onCompactTypeChange: () => void = () => {
    const { compactType: oldCompactType } = this.state
    const compactType =
      oldCompactType === 'horizontal'
        ? 'vertical'
        : oldCompactType === 'vertical'
          ? 'vertical'
          : 'horizontal'
    this.setState({ compactType })
  }

  onResizeTypeChange: () => void = () => {
    const resizeHandles =
      this.state.resizeHandles === availableHandles ? ['se'] : availableHandles
    this.setState({
      resizeHandles,
      layouts: {
        lg: generateLayoutFromDashboardItems(this.props.dashboardItems),
      },
    })
  }

  onLayoutChange = (currentLayout: Layout[], allLayouts: Layouts) => {
    this.props.onLayoutChange(currentLayout, allLayouts)
  }

  onNewLayout: EventHandler<SyntheticEvent> = () => {
    const layout = generateLayoutFromDashboardItems(this.props.dashboardItems)
    this.setState({
      layouts: { lg: layout },
    })
  }

  onDrop: (layout: Layout[], item: Layout, e: Event) => void = (elemParams) => {
    alert(`Element parameters: ${JSON.stringify(elemParams)}`)
  }

  render(): React.ReactNode {
    console.log('rerender dashboard')
    // eslint-disable-next-line no-unused-vars
    return (
      <div>
        <div>
          Current Breakpoint: {this.state.currentBreakpoint} (
          {this.props.cols[this.state.currentBreakpoint]} columns)
        </div>
        <div>
          Compaction type:{' '}
          {_.capitalize(this.state.compactType) || 'No Compaction'}
        </div>
        <button onClick={this.onNewLayout}>Generate New Layout</button>
        <button onClick={this.onCompactTypeChange}>
          Change Compaction Type
        </button>
        <button onClick={this.onResizeTypeChange}>
          Resize{' '}
          {this.state.resizeHandles === availableHandles
            ? 'One Corner'
            : 'All Corners'}
        </button>
        <ResponsiveReactGridLayout
          {...this.props}
          layouts={{
            lg: this.state.layouts.lg.map((l) => l.layout),
          }}
          onBreakpointChange={this.onBreakpointChange}
          onLayoutChange={this.onLayoutChange}
          onDrop={this.onDrop}
          // WidthProvider option
          measureBeforeMount={false}
          // I like to have it animate on mount. If you don't, delete `useCSSTransforms` (it's default `true`)
          // and set `measureBeforeMount={true}`.
          useCSSTransforms={this.state.mounted}
          compactType={this.state.compactType}
          preventCollision={!this.state.compactType}
          /* eslint-disable-next-line react/no-children-prop */
          children={this.generateDOM()}
        />
      </div>
    )
  }
}

const ResponsiveReactGridLayout = WidthProvider(Responsive)

// function generateLayout(resizeHandles: string[]): Array<{
//   x: number
//   y: number
//   w: number
//   h: number
//   i: string
//   resizeHandles: string[]
// }> {
//   return _.map(_.range(0, 25), function (_, i) {
//     const y = Math.ceil(Math.random() * 4) + 1
//     return {
//       x: Math.round(Math.random() * 5) * 2,
//       y: Math.floor(i / 6) * y,
//       w: 2,
//       h: y,
//       i: i.toString(),
//       resizeHandles,
//     }
//   })
// }

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
