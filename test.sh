#!/bin/bash

cargo test --all

wasm-pack test crates/browser --firefox --headless
