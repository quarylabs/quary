const { defineConfig } = require('eslint/config')
const shared = require('@quary/eslint-config-shared')

module.exports = defineConfig([{ files: ['src/**/*.ts'], extends: shared }])
