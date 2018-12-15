#! /usr/bin/env bash

set -e

if [ -z "${1}" ] ; then
    echo branch?
    exit 0
fi

git checkout ${1}
rm -rf target/deploy

cargo clean
cargo clippy -- -D warnings
cargo clean
cargo web build -p rustmith_correlation_worker --target wasm32-unknown-unknown --release
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.js ./frontend/static/
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.wasm ./frontend/static/
cargo clean
cargo web deploy -p rustmith_frontend --target wasm32-unknown-unknown --release

git checkout gh-pages
mv -f target/deploy/* ./

RUSTMITH_CSS_CHECKSUM=$(md5sum index.css | awk '{print $1}' | xargs echo -n)
RUSTMITH_JS_CHECKSUM=$(md5sum rustmith.js | awk '{print $1}' | xargs echo -n)
RUSTMITH_WASM_CHECKSUM=$(md5sum rustmith.wasm | awk '{print $1}' | xargs echo -n)

sed -i "s/rustmith_frontend.js/rustmith_frontend.js?hash=${RUSTMITH_JS_CHECKSUM}/g" index.html
sed -i "s/index.css/index.css?hash=${RUSTMITH_CSS_CHECKSUM}/g" index.html
sed -i "s/rustmith_frontend.wasm/rustmith_frontend.wasm?hash=${RUSTMITH_WASM_CHECKSUM}/g" rustmith_frontend.js

git add *.css
git add *.html
git add *.js
git add *.wasm

git commit -m "update"
git push origin gh-pages
git checkout ${1}
