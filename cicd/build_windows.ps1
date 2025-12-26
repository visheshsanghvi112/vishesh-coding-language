# Vish Language - Windows Build Script
# Created by Vishesh Sanghvi

# create release folder
New-Item -ItemType Directory -Force -Path ./release_windows

# install cargo-wix
cargo install cargo-wix --version 0.3.3

# build the binary
cargo build --package vish --release

# build the installer
cargo wix --package vish --no-build --nocapture --output ./release_windows/vish-win-x86_64-installer.msi

# zip the binary
Compress-Archive -Path ./target/release/vish.exe -DestinationPath ./release_windows/vish-win-x86_64.zip

ls ./release_windows
