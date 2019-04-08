
#!/bin/bash

. ./bootstrap.sh

cd $(dirname $0)

mkdir -p build/
mkdir -p dist/

cp static/index.html build/

wasm-pack build --target no-modules --no-typescript --out-dir ./build
