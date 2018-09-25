var path = require('path')
var webpack = require('webpack')

const PRODUCTION = process.env.NODE_ENV === 'production'
const outputDir = path.join(__dirname, 'public')

module.exports = {
  entry: [
    './src/frontend/index.jsx'
  ],
  mode: PRODUCTION ? 'production' : 'development',
  output: {
    path: outputDir,
    filename: 'js/bundle.js',
    publicPath: outputDir
  },
  resolve: {
    extensions: ['.js', '.jsx', '.css']
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        loader: 'style-loader'
      }, {
        test: /\.css$/,
        loader: 'css-loader',
        query: {
          modules: true,
          localIdentName: '[name]__[local]___[hash:base64:5]'
        }
      }
      ,
      {
        test: /\.jsx?$/,
        exclude: [/node_modules/],
        use: [
          { loader: 'babel-loader' }
        ]
      }
    ]
  }
}
