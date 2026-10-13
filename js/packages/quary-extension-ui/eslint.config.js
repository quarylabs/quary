import { defineConfig } from 'eslint/config'
import shared from '@quary/eslint-config-shared'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import globals from 'globals'

export default defineConfig([
  { ignores: ['src/components/ui/**'] },
  {
    files: ['src/**/*.{ts,tsx}'],
    extends: shared,
    languageOptions: {
      globals: { ...globals.browser, ...globals.es2020 },
    },
    plugins: {
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'react-refresh/only-export-components': 'warn',
    },
  },
])
