import * as path from 'path'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  resolve: {
    alias: {
      '@ui': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, '../quary-extension-bus/src'),
      '@quary/proto': path.resolve(__dirname, '../proto/src/generated'),
    },
  },
})
