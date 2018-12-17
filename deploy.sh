#! /usr/bin/env bash

set -e

if [ -z "${1}" ] ; then
    echo branch?
    exit 0
fi

git checkout ${1}

cargo clean
cargo clippy -- -D warnings
cargo test
cargo web build -p rustmith_correlation_worker --target wasm32-unknown-unknown --release
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.js ./frontend/static/
cp -f target/wasm32-unknown-unknown/release/rustmith_correlation_worker.wasm ./frontend/static/
cargo web deploy -p rustmith_frontend --target wasm32-unknown-unknown --release

git checkout gh-pages
rm *.css
rm *.html
rm *.js
rm *.wasm
mv -f target/deploy/* ./

CSS_CHECKSUM=$(md5sum index.css | awk '{print $1}' | xargs echo -n)
FRONTEND_CHECKSUM=$(md5sum rustmith_frontend.wasm | awk '{print $1}' | xargs echo -n)
WORKER_CHECKSUM=$(md5sum rustmith_correlation_worker.wasm | awk '{print $1}' | xargs echo -n)

sed -i "s/index.css/index.css?hash=${CSS_CHECKSUM}/g" index.html
sed -i "s/rustmith_frontend.js/rustmith_frontend.js?hash=${FRONTEND_CHECKSUM}/g" index.html
sed -i "s/rustmith_frontend.wasm/rustmith_frontend.wasm?hash=${FRONTEND_CHECKSUM}/g" rustmith_frontend.js
sed -i "s/rustmith_correlation_worker.js/rustmith_correlation_worker.js?hash=${WORKER_CHECKSUM}/g" index.html
sed -i "s/rustmith_correlation_worker.wasm/rustmith_correlation_worker.wasm?hash=${WORKER_CHECKSUM}/g" rustmith_correlation_worker.js

git add *.css
git add *.html
git add *.js
git add *.wasm

git commit -m "update"
git push origin gh-pages
git checkout ${1}
