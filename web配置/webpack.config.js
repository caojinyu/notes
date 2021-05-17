// 引入一个包
const path = require('path'); //拼接路径

const HTMLWebpackPlugin = require("html-webpack-plugin");

const { CleanWebpackPlugin } = require("clean-webpack-plugin");

// webpack中的所有的配置信息，都应该写在module.exports里面
module.exports = {
    mode: "production",
    // 指定入口文件
    entry: "./src/index.ts",

    // 指定打包文件所在的目录
    output: {
        // 打包后的目录
        path: path.resolve(__dirname, "dist"),
        // 打包后文件的名字
        filename: "bundle.js"
    },

    // 指定webpack打包时要使用的模块
    module: {
        // 指定loader的规则，加载规则
        rules: [
            {
                // test指定的时规则生效的文件
                test: /\.ts$/,
                // 要使用的loader，针对test的规则
                use: [
                    {
                        // 指定加载器
                        loader: "babel-loader",
                        //配置
                        options: {
                            // 设置预定的环境
                            presets: [
                                [
                                    "@babel/preset-env", //环境
                                    // 配置信息
                                    {
                                        // 要兼容的目标浏览器
                                        targets:{
                                            "chrome": "88",
                                            "safari": "14",
                                        },
                                        // 指定corejs的版本
                                        "corejs": "3",
                                        // 使用corejs的方式 usage按需加载
                                        "useBuiltIns": "usage"
                                    }
                                ]
                            ]
                        }
                    } ,
                    "ts-loader"
                ],

                // 要排除的文件
                exclude: /node_modules/
            },

            // 设置less文件的处理
            {
                test: /\.less$/,
                use: [
                    "style-loader",
                    "css-loader",
                    {
                        loader: "postcss-loader",
                        options: {
                            postcssOptions: {
                                plugins: [
                                    [
                                        "postcss-preset-env",
                                        {
                                            browser: 'last 2 version'
                                        }
                                    ]
                                ]
                            }
                        }
                    },
                    "less-loader"
                ]
            }
        ]
    },

    // 配置插件
    plugins: [
        new CleanWebpackPlugin(),

        new HTMLWebpackPlugin({
            template: "./src/index.html"
        })
    ],

    resolve: {
        extensions: [".ts", ".js"]
    }
}