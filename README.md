# rustmith
Rocksmith clone for Web platform

## Setup

Simply run

```shell
# first time setup
rustup default nightly
rustup update
rustup target add wasm32-unknown-unknown
cargo install --force cargo-web

# during development
cargo web start --auto-reload
# open localhost:8000 in browser
```
