const path = require('path')

/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  moduleNameMapper: {
    '^@shared/(.*)$': path.resolve(__dirname, '../quary-extension-bus/src/$1'),
    '^@quary/proto/(.*)$': path.resolve(__dirname, '../proto/src/generated/$1'),
  },
}
