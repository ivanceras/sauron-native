
cargo build --features "with-nwg" --target=x86_64-pc-windows-gnu
wine ../../target/x86_64-pc-windows-gnu/debug/cross_widget.exe
