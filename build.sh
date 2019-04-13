#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/web_bind.wasm \
    --out-dir ./www/
cd www
npm run-script build
