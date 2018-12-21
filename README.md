# rustmith
Rocksmith clone for Web platform

## Setup

Simply run

```shell
# first time setup
rustup default nightly
rustup update
rustup target add wasm32-unknown-unknown
rustup component add clippy
rustup component add rustfmt
cargo install --force cargo-web

# during development
cargo web start -p rustmith_frontend --auto-reload
# open localhost:8000 in browser
```
