# Wasemu; WebAssembly Emulator

Wasemu はWASM(WebAssembly)の簡易なエミュレータを提供するRustのクレートです。
何らかのアプリケーションに組み込んで擬似的にWASMを実行するためのライブラリとなることを目的にしており、
WASMの文法が提供するホストとの資源の相互活用機能(関数のimport, export, メモリの要求など)を活用し、WASMとホストアプリケーション間の機能の連携を図ります。

Wasemu is a Rust crate which provides with simple WASM(WebAssembly) emulator.
Our target is releasig Wasemu as a library for virtually executing WASM in some application.
For our library usefulness, we will provide with cooperation features between WASM and host application with mutual resources utilization features defined in WASM convention such as importing and exporting functions and reguiring host memory.
