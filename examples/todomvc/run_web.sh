#!/bin/bash

set -v
if wasm-pack build --target no-modules -- --features "with-html"; then
    basic-http-server ./ -a 0.0.0.0:7000
fi
