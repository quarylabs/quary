import { Meta, StoryObj } from '@storybook/react'
import { Dag } from './Dag'

const meta: Meta<typeof Dag> = {
  component: Dag,
}

export default meta

type Story = StoryObj<typeof Dag>

export const Main: Story = {
  args: {
    dag: {
      models: [
        {
          name: 'orders',
          modelOrSeedOrSource: 3,
          filePath: '/path/to/orders.sql',
        },
        {
          name: 'customers',
          modelOrSeedOrSource: 3,
          filePath: '/path/to/customers.sql',
        },
        {
          name: 'products',
          modelOrSeedOrSource: 3,
          filePath: '/path/to/products.sql',
        },
        {
          name: 'inventory',
          modelOrSeedOrSource: 3,
          filePath: '/path/to/inventory.sql',
        },
        {
          name: 'shipping',
          modelOrSeedOrSource: 3,
          filePath: '/path/to/shipping.sql',
        },
        {
          name: 'order_enrichment',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/order_enrichment.sql',
        },
        {
          name: 'customer_segmentation',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/customer_segmentation.sql',
        },
        {
          name: 'product_analysis',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/product_analysis.sql',
        },
        {
          name: 'inventory_analysis',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/inventory_analysis.sql',
        },
        {
          name: 'shipping_analysis',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/shipping_analysis.sql',
        },
        {
          name: 'sales_forecasting',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/sales_forecasting.sql',
        },
        {
          name: 'logistics_optimization',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/logistics_optimization.sql',
        },
        {
          name: 'market_trends',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/market_trends.sql',
        },
        {
          name: 'combined_insights',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/combined_insights.sql',
        },
        {
          name: 'finance_insights',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/finance_insights.sql',
        },
        {
          name: 'operational_insights',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/operational_insights.sql',
        },
        {
          name: 'strategic_decisions',
          modelOrSeedOrSource: 1,
          filePath: '/path/to/strategic_decisions.sql',
        },
      ],
      dag: {
        nodes: [
          { id: 'orders', isCached: false },
          { id: 'customers', isCached: false },
          { id: 'products', isCached: false },
          { id: 'inventory', isCached: false },
          { id: 'shipping', isCached: false },
          { id: 'order_enrichment', isCached: false },
          { id: 'customer_segmentation', isCached: false },
          { id: 'product_analysis', isCached: false },
          { id: 'inventory_analysis', isCached: false },
          { id: 'shipping_analysis', isCached: false },
          { id: 'sales_forecasting', isCached: false },
          { id: 'logistics_optimization', isCached: false },
          { id: 'market_trends', isCached: false },
          { id: 'combined_insights', isCached: true },
          { id: 'finance_insights', isCached: true },
          { id: 'operational_insights', isCached: true },
          { id: 'strategic_decisions', isCached: false },
        ],
        edges: [
          { from: 'orders', to: 'order_enrichment' },
          { from: 'customers', to: 'customer_segmentation' },
          { from: 'products', to: 'product_analysis' },
          { from: 'inventory', to: 'inventory_analysis' },
          { from: 'shipping', to: 'shipping_analysis' },
          { from: 'order_enrichment', to: 'sales_forecasting' },
          { from: 'customer_segmentation', to: 'market_trends' },
          { from: 'product_analysis', to: 'market_trends' },
          { from: 'inventory_analysis', to: 'logistics_optimization' },
          { from: 'shipping_analysis', to: 'logistics_optimization' },
          { from: 'sales_forecasting', to: 'combined_insights' },
          { from: 'market_trends', to: 'combined_insights' },
          { from: 'logistics_optimization', to: 'operational_insights' },
          { from: 'combined_insights', to: 'finance_insights' },
          { from: 'operational_insights', to: 'strategic_decisions' },
          { from: 'finance_insights', to: 'strategic_decisions' },
        ],
      },
    },
  },
}
