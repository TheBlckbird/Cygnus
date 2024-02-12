#!/bin/bash

targets=("aarch64-apple-darwin" "x86_64-apple-darwin")

for target in "${targets[@]}"; do
    cargo build --target $target --release
done
