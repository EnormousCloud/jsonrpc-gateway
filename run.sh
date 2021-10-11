#!/usr/bin/env bash

set -xe

run_redis() {
    docker run -d -p 6379:6379 --name redis redis:latest
}

app_build() {
    cd jsonrpc-app
    cargo build --release
    cd ..
    cd jsonrpc-key
    cargo build --release
    cd ..
    cd jsonrpc-gw
    cargo build --release
    cd ..
}

app_install() {
    cp ./target/release/jsonrpc-app ~/.cargo/bin/jsonrpc-app
    cp ./target/release/jsonrpc-key ~/.cargo/bin/jsonrpc-key
    cp ./target/release/jsonrpc-gw ~/.cargo/bin/jsonrpc-gw
}

[[ "$1" == "redis" ]] && { shift; run_redis; }
[[ "$1" == "build" ]] && { shift; app_build; }
[[ "$1" == "install" ]] && { shift; app_install; }

