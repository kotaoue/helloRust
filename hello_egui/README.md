# hello_egui

This is a simple Hello World GUI application built with the [egui](https://github.com/emilk/egui) framework.

## Features

egui の主な特徴：

* **Pure Rust** — 外部の C/C++ ライブラリに依存せず、Rust だけで実装されている
* **即時モード (Immediate mode)** — フレームごとに UI を描画する即時モード GUI。状態管理がシンプル
* **クロスプラットフォーム** — Linux・macOS・Windows・WebAssembly (WASM) で動作する
* **軽量・高速** — 依存クレートが少なく、コンパイルが速い
* **カスタマイズ性** — テーマ・フォント・レイアウトを柔軟に変更できる
* **eframe との統合** — `eframe` クレートを使うことで、ネイティブアプリと Web アプリを同一コードで配信できる

## Usage

```sh
cd hello_egui
cargo run
Hello, World!
```

## References

* [egui - an easy-to-use GUI in pure Rust](https://github.com/emilk/egui)
* [eframe - the official egui framework](https://docs.rs/eframe/latest/eframe/)
