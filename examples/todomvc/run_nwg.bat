#
# sudo apt install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64
# rustup target add x86_64-pc-windows-gnu
# cargo build --release --target=x86_64-pc-windows-gnu
# cargo build --release --target=x86_64-pc-windows-gnu --example basic
# wine target/x86_64-pc-windows-gnu/release/examples/basic.exe

cargo run --features "with-nwg"
# cargo build --features "with-nwg" --target=x86_64-pc-windows-gnu

# If there is an error occured, such as err:module:import_dll Library libstdc++-6.dll
# Copy the files to the same directory of the build exe files
# cp /usr/lib/gcc/x86_64-w64-mingw32/7.3-win32/libstdc++-6.dll target/target/x86_64-pc-windows-gnu/release/examples
# cp /usr/lib/gcc/x86_64-w64-mingw32/7.3-win32/libgcc_s_seh-1.dll target/target/x86_64-pc-windows-gnu/release/examples
