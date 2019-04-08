
#!/bin/bash

. ./build-wasm.sh

if ! type basic-http-server > /dev/null; then
   cargo install basic-http-server
fi
basic-http-server ./build/
