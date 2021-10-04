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

[[ "$1" == "redis" ]] && { shift; run_redis; }
[[ "$1" == "build" ]] && { shift; app_build; }

