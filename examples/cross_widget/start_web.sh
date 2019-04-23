#!/bin/bash

set -v
wasm-pack build --target no-modules -- --features "with-html"

basic-http-server ./ -a 0.0.0.0:5000
