var path = require('path')
var webpack = require('webpack')

const PRODUCTION = process.env.NODE_ENV === 'production'
const outputDir = path.join(__dirname, 'public')

module.exports = {
  devtool: 'eval',
  entry: [
    './src/frontend/index'
  ],
  mode: PRODUCTION ? 'production' : 'development',
  output: {
    path: outputDir,
    filename: 'js/bundle.js',
    publicPath: outputDir,
  },
  resolve: {
    extensions: ['.js', '.jsx']
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          { loader: 'css-loader' },
        ]
      },
      {
        test: /\.jsx?$/,
        use: [
          { loader: 'babel-loader' },
        ]
      }
    ]
  }
}