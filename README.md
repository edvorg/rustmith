# rustmith
Rocksmith clone for Web platform

## Setup

First time setup:

```shell
rustup default nightly
rustup update
rustup target add wasm32-unknown-unknown
rustup component add clippy
rustup component add rustfmt
cargo install --force cargo-web
cargo install --force cargo-watch
```

During development:

```shell
# build correlation worker for sound processing
./build-worker.sh

# build frontend
cargo web start -p rustmith_frontend --auto-reload

# open localhost:8000 in browser
```
