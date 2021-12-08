# nile-core
Implements the core game logic including a CPU opponent in [brute.rs](src/ai/brute.rs).

## dependencies
* [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) for WASM interaction
* [rand](https://docs.rs/rand/latest/rand/) for shuffling tiles and determining which player goes first
* [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) for crash reporting
* [smallvec](https://github.com/servo/rust-smallvec) for stack-based arrays
