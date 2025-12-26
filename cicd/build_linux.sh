#!/bin/bash
# Vish Language - Linux Build Script
# Created by Vishesh Sanghvi

echo "::group::Set-up Build Dependencies"
# Create release folder
mkdir -p release_linux

# Install Dependencies
sudo apt update
sudo apt upgrade -y
sudo apt install -y gcc-aarch64-linux-gnu
sudo apt install -y binutils-aarch64-linux-gnu
sudo apt install -y gcc-arm-linux-gnueabihf
sudo apt install -y binutils-arm-linux-gnueabihf
sudo apt install -y musl-tools
sudo apt install -y gcc-i686-linux-gnu

rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add i686-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

cargo install cargo-deb
cargo install cross --git https://github.com/cross-rs/cross
echo "::endgroup::"

echo "::group::Building Debian package"
# Build the Debian package for (default : GNU x86_64)
cargo deb -p vish
cp ./target/debian/vish_*.deb ./release_linux/vish-linux-x86_64.deb
echo "::endgroup::"

# Build the binary
build_binary() {
   echo "::group::Building $2"
   rm -rf target
   if [ $1 == "cross" ]; then
      cross build --package vish --release --target $2
   else
      cargo build --package vish --release --target $2
   fi
   if [ -f "./target/$2/release/vish" ]; then
      tar -czvf ./release_linux/$3 -C ./target/$2/release vish
      echo -e "Build completed ./release_linux/$3 Generated\n"
   else
      echo -e "Build failed $3\n"
   fi
   echo "::endgroup::"
}

build_binary default x86_64-unknown-linux-gnu vish-linux-gnu-x86_64.tar.xz
build_binary default aarch64-unknown-linux-gnu vish-linux-gnu-aarch64.tar.xz
build_binary default armv7-unknown-linux-gnueabihf vish-linux-gnueabihf-armv7.tar.xz
build_binary default i686-unknown-linux-gnu vish-linux-gnu-i686.tar.xz
build_binary default x86_64-unknown-linux-musl vish-linux-musl-x86_64.tar.xz
build_binary cross aarch64-unknown-linux-musl vish-linux-musl-aarch64.tar.xz
