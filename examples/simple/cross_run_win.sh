
cargo build --features "with-nwg" --target=x86_64-pc-windows-gnu
wine ../../target/x86_64-pc-windows-gnu/debug/simple.exe
