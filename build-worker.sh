#! /bin/bash

set -e

cargo web build -p rustmith_correlation_worker --target wasm32-unknown-unknown --release
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.js ./frontend/static/
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.wasm ./frontend/static/
