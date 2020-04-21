const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

const appConfig = {
  mode: "production",
  entry: {
    index: "./js/index.tsx",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    contentBase: dist,
  },
  // Enable sourcemaps for debugging webpack's output.
  // devtool: "source-map",
  resolve: {
    // Add '.ts' and '.tsx' as resolvable extensions.
    extensions: [".js", ".ts", ".tsx"],
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.ts(x?)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader",
          },
        ],
      },
      // // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
      // {
      //   enforce: "pre",
      //   test: /\.js$/,
      //   loader: "source-map-loader",
      // },
    ],
  },
  plugins: [new CopyPlugin([path.resolve(__dirname, "static")])],
};

const workerConfig = {
  entry: "./js/worker.ts",
  target: "webworker",
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      forceWatch: false,
    }),
  ],
  resolve: {
    extensions: [".js", ".wasm", ".ts"],
  },
  module: {
    rules: [
      {
        test: /\.ts(x?)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader",
          },
        ],
      },
      // // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
      // {
      //   enforce: "pre",
      //   test: /\.js$/,
      //   loader: "source-map-loader",
      // },
    ],
  },
  output: {
    path: dist,
    filename: "worker.js",
  },
};

module.exports = [appConfig, workerConfig];
