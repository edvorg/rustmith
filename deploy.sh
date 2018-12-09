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
cargo web build --bin correlation_worker --target wasm32-unknown-unknown --release
cp -f target/wasm32-unknown-unknown/release/correlation_worker.js ./static/
cp -f target/wasm32-unknown-unknown/release/correlation_worker.wasm ./static/
cargo web deploy --release

git checkout gh-pages
mv -f target/deploy/* ./

RUSTMITH_CSS_CHECKSUM=$(md5sum index.css | awk '{print $1}' | xargs echo -n)
RUSTMITH_JS_CHECKSUM=$(md5sum rustmith.js | awk '{print $1}' | xargs echo -n)
RUSTMITH_WASM_CHECKSUM=$(md5sum rustmith.wasm | awk '{print $1}' | xargs echo -n)

sed -i "s/rustmith.js/rustmith.js?hash=${RUSTMITH_JS_CHECKSUM}/g" index.html
sed -i "s/index.css/index.css?hash=${RUSTMITH_CSS_CHECKSUM}/g" index.html
sed -i "s/rustmith.wasm/rustmith.wasm?hash=${RUSTMITH_WASM_CHECKSUM}/g" rustmith.js

git add *.css
git add *.html
git add *.js
git add *.wasm

git commit -m "update"
git push origin gh-pages
git checkout ${1}
