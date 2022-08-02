const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  resolve:{
    extensions:[".ts",".js"],
  },
  module:{
    rules:[
      {
        test:/\.ts$/,
        loader:"ts-loader",
      },
    ],
  },
  mode: "development",
  plugins: [
    // new CopyWebpackPlugin(['./src/index.html'])
    new CopyWebpackPlugin(
      [
        "./src/index.html",
        {
          from:"./src/textures",
          to:"./textures",
        },
      ]
    )
  ],
};
