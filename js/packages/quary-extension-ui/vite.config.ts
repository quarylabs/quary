import * as fs from 'fs'
import path from 'path'
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

const OUTPUT_PREFIX = '../quary-extension/src/ui'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, '../quary-extension-bus/src'),
      '@quary/proto': path.resolve(__dirname, '../proto/src/generated'),
    },
  },
  build: {
    emptyOutDir: true,
    outDir: OUTPUT_PREFIX,
    rollupOptions: {
      plugins: [
        {
          name: 'rename js to txt',
          writeBundle: () => {
            const files = fs.readdirSync(OUTPUT_PREFIX + '/assets')
            files.forEach((file) => {
              if (file.endsWith('.js')) {
                fs.renameSync(
                  OUTPUT_PREFIX + `/assets/${file}`,
                  OUTPUT_PREFIX + `/assets/${file.replace('.js', '.js.txt')}`,
                )
              }
              if (file.endsWith('.css')) {
                fs.renameSync(
                  OUTPUT_PREFIX + `/assets/${file}`,
                  OUTPUT_PREFIX + `/assets/${file.replace('.css', '.css.txt')}`,
                )
              }
            })
          },
        },
      ],
      output: {
        entryFileNames: `assets/[name].js`,
        chunkFileNames: `assets/[name].js`,
        assetFileNames: `assets/[name].[ext]`,
      },
    },
  },
})
