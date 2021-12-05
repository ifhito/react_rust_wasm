const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
module.exports = {
    mode: 'development',
    // メインとなるJavaScriptファイル（エントリーポイント）
    entry: './src/index.tsx',
    experiments: {
        asyncWebAssembly: true,
    },
    resolve: {
        extensions: ['.js', '.ts','.tsx','.css']
    },
    ignoreWarnings: [
        (warning) =>
          warning.message ===
          "Critical dependency: the request of a dependency is an expression",
    ],
    module: {
        rules: [
            {
                test: /\.(ts|tsx|js)$/,
                exclude: /node_modules/,
                use: [
                  {
                    loader: 'babel-loader',
                  },
                ],
            },
            {
                test: /\.css$/,
                use: [
                    'css-modules-typescript-loader',
                    {
                      loader: 'css-loader',
                      options: {
                        modules: true
                      }
                    }
                  ]
            },
            { 
                test: /\.wasm$/, // only load WASM files (ending in .wasm)
                // only files in our src/ folder
                include: path.resolve(__dirname, "src"), 
                use: [{ 
                    // load and use the wasm-loader dictionary
                    loader: require.resolve("wasm-loader"), 
                    options: {} 
                 }],
             },
        ]
    },
     // ファイルの出力設定
    output: {
        //  出力ファイルのディレクトリ名
        path: `${__dirname}/dist`,
        // 出力ファイル名
        filename: "main.js"
    },
    plugins:[
        new HtmlWebpackPlugin({ template: './src/index.html' }),
    ]
}