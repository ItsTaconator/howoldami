#!/bin/sh
# This requires cross since it's the easiest way to cross-compile
if ! command -v cross >/dev/null 2>&1
then
	printf "cross is required by this build script"
	exit
fi
cargo clean
# Build for AArch64 MUSL (example: Alpine Linux on a Pi)
echo "Building for AArch64 Linux MUSL"
cross build --target aarch64-unknown-linux-musl -r
# Build for AArch64 GNU (example: Debian on a Pi)
echo "Building for AArch64 Linux GNU"
cross build --target aarch64-unknown-linux-gnu -r
# Build for x64 Windows
echo "Building for x64 Windows"
cross build --target x86_64-pc-windows-gnu -r
# Build for native architecture
cargo build -r
