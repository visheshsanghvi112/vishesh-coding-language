#!/bin/bash
# Vish Language - macOS Build Script
# Created by Vishesh Sanghvi

# Create release folder
mkdir -p release_mac

# Build binary (x86_64 - Intel Mac)
cargo build --package vish --release --target x86_64-apple-darwin
tar -czvf ./release_mac/vish-darwin-x86_64.tar.gz -C ./target/x86_64-apple-darwin/release vish

# Build binary (aarch64 - Apple Silicon)
rustup target add --toolchain stable aarch64-apple-darwin
cargo build --package vish --release --target aarch64-apple-darwin
tar -czvf ./release_mac/vish-darwin-aarch64.tar.gz -C ./target/aarch64-apple-darwin/release vish

echo "macOS builds complete!"
ls ./release_mac
