#!/bin/sh
cargo clean
# Build for AArch64 MUSL (example: Alpine Linux on a Pi)
cross build --target aarch64-unknown-linux-musl -r
# Build for AArch64 GNU (example: Debian on a Pi)
cross build --target aarch64-unknown-linux-gnu -r
# Build for native architecture
cargo build --release