
const path = require('path');
module.exports = {
  entry: "./paint.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "paint.js",
  },
  mode: "development"
};