/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

//@ts-check
'use strict'

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig **/

const path = require('path')
const webpack = require('webpack')

module.exports = (
  /** @type {{ STAGE: 'production' | 'development'; }} */ env,
) => {
  const { STAGE } = env
  /** @type WebpackConfig */
  const webExtensionConfig = {
    mode: 'none', // this leaves the source code as close as possible to the original (when packaging we set this to 'production')
    target: 'webworker', // extensions run in a webworker context
    entry: {
      extension: './src/web/extension.ts',
    },
    output: {
      filename: '[name].js',
      path: path.join(__dirname, './dist/web'),
      libraryTarget: 'commonjs',
      devtoolModuleFilenameTemplate: '../../[resource-path]',
    },
    resolve: {
      mainFields: ['browser', 'module', 'main'], // look for `browser` entry point in imported node modules
      extensions: ['.ts', '.js', '.wasm'], // support ts-files and js-files
      alias: {
        '@shared': path.resolve(__dirname, '../quary-extension-bus/src'),
        '@quary/proto': path.resolve(__dirname, '../proto/src/generated'),
      },
      fallback: {
        // Webpack 5 no longer polyfills Node.js core modules automatically.
        // see https://webpack.js.org/configuration/resolve/#resolvefallback
        // for the list of Node.js core module polyfills.
        child_process: false, // polyfilling child_process for web
        assert: require.resolve('assert'),
        'process/browser': require.resolve('process/browser'),
        stream: require.resolve('stream-browserify'),
        path: false,
        fs: false,
        crypto: require.resolve('crypto-browserify'),
        request: false,
        buffer: require.resolve('buffer'),
        vm: require.resolve('vm-browserify'),
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
        maxChunks: 1, // disable chunks by default since web extensions must be a single bundle
      }),
      new webpack.ProvidePlugin({
        process: 'process/browser', // provide a shim for the global `process` variable
      }),
      new webpack.ProvidePlugin({
        Buffer: ['buffer', 'Buffer'],
      }),
      new webpack.DefinePlugin({
        __MODE__: JSON.stringify(STAGE),
      }),
    ],
    externals: {
      vscode: 'commonjs vscode', // ignored because it doesn't exist
    },
    performance: {
      hints: false,
    },
    devtool: 'nosources-source-map', // create a source map that points to the original source file
    infrastructureLogging: {
      level: 'log', // enables logging required for problem matchers
    },
  }

  return [webExtensionConfig]
}
