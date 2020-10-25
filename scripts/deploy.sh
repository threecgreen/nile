#! /bin/sh
cargo install wasm-pack
wasm-pack build --release
cargo run --bin gen_constants
cd web
npm ci
npm run build
