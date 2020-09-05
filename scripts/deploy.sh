#! /bin/sh
cargo install wasm-pack
wasm-pack build --release
cd web
npm ci
npm run build
