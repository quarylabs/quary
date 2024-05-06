/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

//@ts-check
'use strict'

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig */

const path = require('path')
const webpack = require('webpack')

module.exports = (
  /** @type {{ STAGE: 'production' | 'development'; }} */ env,
) => {
  const { STAGE } = env
  /** @type WebpackConfig */
  const nodeExtensionConfig = {
    mode: 'none',
    resolve: {
      mainFields: ['browser', 'module', 'main'],
      extensions: ['.ts', '.js', '.wasm'],
      alias: {
        '@shared': path.resolve(__dirname, '../quary-extension-bus/src'),
        '@quary/proto': path.resolve(__dirname, '../proto/src/generated'),
      },
      fallback: {
        assert: require.resolve('assert'),
        'process/browser': require.resolve('process/browser'),
        stream: require.resolve('stream-browserify'),
        path: false,
        fs: false,
        crypto: require.resolve('crypto-browserify'),
        request: false,
        buffer: require.resolve('buffer'),
      },
    },
    module: {
      rules: [
        {
          test: /\.ts$/,
          exclude: /node_modules/,
          use: [
            {
              loader: 'ts-loader',
            },
          ],
        },
        {
          test: /\.wasm$/,
          type: 'asset/inline',
        },
        {
          test: /\.txt$/,
          use: [
            {
              loader: 'raw-loader',
            },
          ],
        },
      ],
    },
    plugins: [
      new webpack.optimize.LimitChunkCountPlugin({
        maxChunks: 1,
      }),
      new webpack.ProvidePlugin({
        process: 'process/browser',
      }),
      new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
      }),
      new webpack.DefinePlugin({
        __MODE__: JSON.stringify(STAGE),
      }),
    ],
    externals: {
      vscode: 'commonjs vscode',
    },
    performance: {
      hints: false,
    },
    devtool: 'nosources-source-map',
    infrastructureLogging: {
      level: 'log',
    },
    target: 'node',
    entry: {
      'node/extension': './src/web/extension.ts',
    },
    output: {
      filename: '[name].js',
      path: path.join(__dirname, './dist'),
      libraryTarget: 'commonjs',
      devtoolModuleFilenameTemplate: '../../[resource-path]',
    },
  }

  return [nodeExtensionConfig]
}
