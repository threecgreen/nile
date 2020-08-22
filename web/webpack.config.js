const CopyWebpackPlugin = require("copy-webpack-plugin");
const TsConfigPathsPlugin = require("tsconfig-paths-webpack-plugin");
const path = require('path');

module.exports = (_env, argv) => {
    const isProd = argv.mode == "production" || argv.mode == "p";

    return {
        devtool: isProd ? false : "inline-source-map",
        entry: "./bootstrap.ts",
        module: {
            rules: [
                {
                    test: /\.(tsx?)$/,
                    use: [
                        {
                            loader: "ts-loader",
                            options: { configFile: isProd ? "tsconfig.prod.json" : "tsconfig.json" },
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
                {
                    test: /\.css$/i,
                    use: [
                        'style-loader',
                        {
                            loader: 'css-loader',
                            options: {
                                // Run `postcss-loader` on each CSS `@import`, do not forget that `sass-loader` compile non CSS `@import`'s into a single file
                                // If you need run `sass-loader` and `postcss-loader` on each CSS `@import` please set it to `2`
                                importLoaders: 1,
                                // Automatically enable css modules for files satisfying `/\.module\.\w+$/i` RegExp.
                                modules: { auto: true },
                                sourceMap: !isProd,
                            },
                        },
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
            new CopyWebpackPlugin({
                patterns: ['index.html', 'index.css']
            })
        ],
        resolve: {
            plugins: [new TsConfigPathsPlugin({ configFile: isProd ? "tsconfig.prod.json" : "tsconfig.json" })],
            extensions: [".js", ".json", ".ts", ".tsx"],
        },
    };
};
