
const path = require('path');

module.exports = {
  entry: {
    "gif.worker": "./node_modules/gif.js/dist/gif.worker.js",
    index: "./js/bootstrap.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: '[name].js',
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.(glsl|frag|vert)$/,
        use: "raw-loader",
        exclude: /node_modules/
      },
      {
        test: /\.(glsl|frag|vert)$/,
        use: "glslify-loader",
        exclude: /node_modules/
      },
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env", "@babel/preset-react"],
            plugins: ["@babel/plugin-syntax-dynamic-import"]
          }
        }
      }
    ]
  }
};