
#!/bin/bash


cd $(dirname $0)

rm -rf dist/
mkdir -p dist/

. ./copy_files.sh

if . ./bootstrap.sh; then
    wasm-pack build --target no-modules --no-typescript --out-dir ./dist --release -- --features wee_alloc
fi
