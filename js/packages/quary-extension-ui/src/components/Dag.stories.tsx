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
          assetType: 3,
          filePath: '/path/to/orders.sql',
          tags: [],
        },
        {
          name: 'customers',
          assetType: 3,
          filePath: '/path/to/customers.sql',
          tags: [],
        },
        {
          name: 'products',
          assetType: 3,
          filePath: '/path/to/products.sql',
          tags: [],
        },
        {
          name: 'inventory',
          assetType: 3,
          filePath: '/path/to/inventory.sql',
          tags: [],
        },
        {
          name: 'shipping',
          assetType: 3,
          filePath: '/path/to/shipping.sql',
          tags: [],
          columns: [],
        },
        {
          name: 'order_enrichment',
          assetType: 1,
          filePath: '/path/to/order_enrichment.sql',
          tags: [],
        },
        {
          name: 'customer_segmentation',
          assetType: 1,
          filePath: '/path/to/customer_segmentation.sql',
          tags: [],
        },
        {
          name: 'product_analysis',
          assetType: 1,
          filePath: '/path/to/product_analysis.sql',
          tags: [],
        },
        {
          name: 'inventory_analysis',
          assetType: 1,
          filePath: '/path/to/inventory_analysis.sql',
          tags: [],
        },
        {
          name: 'shipping_analysis',
          assetType: 1,
          filePath: '/path/to/shipping_analysis.sql',
          tags: [],
        },
        {
          name: 'sales_forecasting',
          assetType: 1,
          filePath: '/path/to/sales_forecasting.sql',
          tags: [],
        },
        {
          name: 'logistics_optimization',
          assetType: 1,
          tags: [],
          filePath: '/path/to/logistics_optimization.sql',
        },
        {
          name: 'market_trends',
          assetType: 1,
          tags: [],
          filePath: '/path/to/market_trends.sql',
        },
        {
          name: 'combined_insights',
          assetType: 1,
          filePath: '/path/to/combined_insights.sql',
          tags: [],
        },
        {
          name: 'finance_insights',
          assetType: 1,
          filePath: '/path/to/finance_insights.sql',
          tags: [],
        },
        {
          name: 'operational_insights',
          assetType: 1,
          filePath: '/path/to/operational_insights.sql',
          tags: [],
        },
        {
          name: 'strategic_decisions',
          assetType: 1,
          filePath: '/path/to/strategic_decisions.sql',
          tags: [],
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
