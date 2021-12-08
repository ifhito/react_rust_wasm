# react_rust_wasm

## reactを使ってwasmを行う

[Notion(途中)](https://solstice-save-d11.notion.site/react-wasm-8278b24c3e354f41812492a439638783)

# 参考文献
## 実装の参考
[Intro to Webassembly in React With Rust](https://medium.com/swlh/intro-to-webassembly-in-react-with-rust-d067408231b9)

[Rustで画像に文字を描画する](https://zenn.dev/corocn/articles/69a6735c10ce4f)

[Rust で画像処理？](https://qiita.com/RotaryKer/items/a9897ddccdd0d8e0a1c2)

[WebAssembly (Rust) によるブラウザ上の画像処理](https://qiita.com/toyohisa/items/6e9336ff5a86911369ef)

[Rustで画像をネガポジ変換](https://doitu.info/blog/5badf7568dbc7a000fc280c6)

[Rust - image](https://docs.rs/image/latest/image/)

## 補足情報など

[Rust(wasm)のimage::load_from_memory遅すぎ問題](https://zenn.dev/dozo/articles/14bc23b488c95a)

[WebAssembly で画像のリサイズ処理をやってみたら JavaScript + Canvas API より遅かった話](https://qiita.com/yokra9/items/f9e98a9b47fe2d1234b0)

## エラー解消の参考

- wasmでwebpack5だとwarningが出る

[This crate hits a buggy Webpack 5 warning when building for wasm32-unknown](https://github.com/rust-random/getrandom/issues/224)

- babelがReferenceError: regeneratorRuntime is not definedのエラーを出す

[Node.js + Babelで「ReferenceError: regeneratorRuntime is not defined」となる場合](https://www.aizulab.com/blog/referenceerror-regeneratorruntime-is-not-defined/)

- cssのtypeエラー

[How to import CSS modules with Typescript, React and Webpack](https://stackoverflow.com/questions/41336858/how-to-import-css-modules-with-typescript-react-and-webpack)

- jpgが読まれないエラー

[The global thread pool has not been initialized.: ThreadPoolBuildError { kind: IOError(Error { kind: Unsupported, message: "operation not supported on this platform" }) } ](https://github.com/image-rs/image/issues/1496)
