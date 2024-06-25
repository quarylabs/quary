import { Layout, Responsive, WidthProvider } from 'react-grid-layout'
import _ from 'lodash'
import { EventHandler, Component, SyntheticEvent } from 'react'

interface Props {
  className: string
  cols: Record<string, number>
  onLayoutChange: Function
  rowHeight: number
}

interface State {
  currentBreakpoint: string
  compactType: CompactType
  mounted: boolean
  resizeHandles: string[]
  layouts: Record<string, Layout>
}

export type CompactType = 'horizontal' | 'vertical'

const availableHandles = ['s', 'w', 'e', 'n', 'sw', 'nw', 'se', 'ne']

const ResponsiveReactGridLayout = WidthProvider(Responsive)

export default class Dashboard extends Component<Props, State> {
  static defaultProps: Props = {
    className: 'layout',
    rowHeight: 30,
    onLayoutChange() {},
    cols: { lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 },
  }

  state: State = {
    currentBreakpoint: 'lg',
    compactType: 'vertical',
    resizeHandles: ['se'],
    mounted: false,
    layouts: { lg: generateLayout(['se']) },
  }

  componentDidMount() {
    this.setState({ mounted: true })
  }

  generateDOM(): React.ReactNodeArray {
    return _.map(this.state.layouts.lg, function (l, i) {
      return (
        <div key={i} className={l.static ? 'static' : ''}>
          {l.static ? (
            <span
              className="text"
              title="This item is static and cannot be removed or resized."
            >
              Static - {i}
            </span>
          ) : (
            <span className="text">{i}</span>
          )}
        </div>
      )
    })
  }

  onBreakpointChange: (Breakpoint) => void = (breakpoint) => {
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
          ? null
          : 'horizontal'
    this.setState({ compactType })
  }

  onResizeTypeChange: () => void = () => {
    const resizeHandles =
      this.state.resizeHandles === availableHandles ? ['se'] : availableHandles
    this.setState({
      resizeHandles,
      layouts: { lg: generateLayout(resizeHandles) },
    })
  }

  onLayoutChange = (layout, layouts) => {
    this.props.onLayoutChange(layout, layouts)
  }

  onNewLayout: EventHandler<SyntheticEvent> = () => {
    this.setState({
      layouts: { lg: generateLayout(this.state.resizeHandles) },
    })
  }

  onDrop: (layout: Layout[], item: Layout, e: Event) => void = (elemParams) => {
    alert(`Element parameters: ${JSON.stringify(elemParams)}`)
  }

  render(): React.ReactNode {
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
          layouts={this.state.layouts}
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
          children={this.generateDOM()}
        />
      </div>
    )
  }
}

function generateLayout(resizeHandles) {
  return _.map(_.range(0, 25), function (item, i) {
    const y = Math.ceil(Math.random() * 4) + 1
    return {
      x: Math.round(Math.random() * 5) * 2,
      y: Math.floor(i / 6) * y,
      w: 2,
      h: y,
      i: i.toString(),
      static: Math.random() < 0.05,
      resizeHandles,
    }
  })
}
