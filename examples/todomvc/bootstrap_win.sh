
#sudo apt install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64
#rustup target add x86_64-pc-windows-gnu


# If there is an error occured, such as err:module:import_dll Library libstdc++-6.dll
# Copy the files to the same directory of the build exe files
copy_file_if_not_in_destination(){
    if test -f "$2"; then
        echo "$2 exists."
    else
        cp "$1" $2
    fi
}

MING_DIR="/usr/lib/gcc/x86_64-w64-mingw32/7.3-win32"
TARGET_DIR="../../target/x86_64-pc-windows-gnu/debug"

copy_file_if_not_in_destination $MING_DIR/libstdc++-6.dll $TARGET_DIR/libstdc++-6.dll
copy_file_if_not_in_destination $MING_DIR/libgcc_s_seh-1.dll $TARGET_DIR/libgcc_s_seh-1.dll
