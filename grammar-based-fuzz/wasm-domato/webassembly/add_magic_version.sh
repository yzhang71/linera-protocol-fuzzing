#!/bin/bash

input_dir="/home/dennis/Documents/Rust_Project/nearcore-fuzz/grammar-based-fuzz/wasm-domato/webassembly/serialized_output/"
output_dir="/home/dennis/Documents/Rust_Project/nearcore-fuzz/grammar-based-fuzz/wasm-domato/webassembly/magic_version_pass/"

if [ ! -d "$output_dir" ]; then
    mkdir -p "$output_dir"
fi

for file_path in "$input_dir"*
do
    if [ -f "$file_path" ]; then
        file_name=$(basename "$file_path")
        { echo -n -e '\x00asm\x01\x00\x00\x00'; cat "$file_path"; } > "$output_dir$file_name"
    fi
done

