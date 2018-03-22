build: format
    bootimage --target x86_64-rust_os

format:
    cargo fmt

run: build
    qemu-system-x86_64  -drive format=raw,file=bootimage.bin -nographic -vnc :0
