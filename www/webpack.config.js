/* jshint esversion: 6 */

const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const TextEncoder = require('text-encoding').TextEncoder;

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            template: 'index.html',
        })
        // new WasmPackPlugin({
        //     crateDirectory: path.resolve(__dirname, "../"),
        //     extraArgs: '--out-dir www/pkg',
        // })
    ],
    mode: 'development'
};
