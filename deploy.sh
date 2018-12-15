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

CSS_CHECKSUM=$(md5sum index.css | awk '{print $1}' | xargs echo -n)
JS_CHECKSUM=$(md5sum rustmith_frontend.js | awk '{print $1}' | xargs echo -n)
WASM_CHECKSUM=$(md5sum rustmith_frontend.wasm | awk '{print $1}' | xargs echo -n)
CORRELATION_WORKER_WASM_CHECKSUM=$(md5sum rustmith_correlation_worker.wasm | awk '{print $1}' | xargs echo -n)

sed -i "s/rustmith_frontend.js/rustmith_frontend.js?hash=${JS_CHECKSUM}/g" index.html
sed -i "s/index.css/index.css?hash=${CSS_CHECKSUM}/g" index.html
sed -i "s/rustmith_frontend.wasm/rustmith_frontend.wasm?hash=${WASM_CHECKSUM}/g" rustmith_frontend.js
sed -i "s/rustmith_correlation_worker.wasm/rustmith_correlation_worker.wasm?hash=${CORRELATION_WORKER_WASM_CHECKSUM}/g" rustmith_correlation_worker.js

git add *.css
git add *.html
git add *.js
git add *.wasm

git commit -m "update"
git push origin gh-pages
git checkout ${1}
