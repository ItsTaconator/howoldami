#!/bin/sh
cargo clean
# Build for AArch64 MUSL (example: Alpine Linux on a Pi)
cross build --target aarch64-unknown-linux-musl -r
# Build for AArch64 GNU (example: Debian on a Pi)
cross build --target aarch64-unknown-linux-gnu -r
# Build for x64 Windows
cross build --target x86_64-pc-windows-gnu
# Build for AArch64 Mac
cross build --target aarch64-apple-darwin
# Build for native architecture
cargo build --release
