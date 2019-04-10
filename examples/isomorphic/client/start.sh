
#!/bin/bash


if . ./build-wasm.sh; then

    if ! type basic-http-server > /dev/null; then
       cargo install basic-http-server
    fi

    basic-http-server ./build/

fi
