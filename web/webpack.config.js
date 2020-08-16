const CopyWebpackPlugin = require("copy-webpack-plugin");
const TsConfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.ts",
    module: {
        rules: [
            {
                test: /\.(tsx?)$/,
                use: [
                    {
                        loader: "ts-loader",
                        options: { configFile: "tsconfig.json" },
                    }
                ]
            },
            {
                test: /\.svg$/,
                use: [
                    {
                        loader: '@svgr/webpack',
                        // options: { typescript: true }
                    }
                ],
            },
        ],
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
    resolve: {
        plugins: [new TsConfigPathsPlugin({ configFile: "tsconfig.json" })],
        extensions: [".js", ".json", ".ts", ".tsx"],
    },
};
