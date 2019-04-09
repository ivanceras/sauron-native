#!/bin/bash

cargo test --all


# Install wasm-pack if it isn't installed yet
if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

wasm-pack test crates/browser --firefox --headless
