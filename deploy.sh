#! /usr/bin/env bash

set -e

if [ -z "${1}" ] ; then
    echo branch?
    exit 0
fi

git checkout ${1}
rm -rf target/deploy

cargo web build --bin correlation_worker --target wasm32-unknown-unknown
cp -f target/wasm32-unknown-unknown/release/correlation_worker.js ./static/
cp -f target/wasm32-unknown-unknown/release/correlation_worker.wasm ./static/
cargo web deploy

git checkout gh-pages
mv -f target/deploy/* ./

git add *.css
git add *.html
git add *.js
git add *.wasm

git commit -m "update"
git push origin gh-pages
git checkout ${1}