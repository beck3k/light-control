#!/bin/bash

# Build the Rust binaries
cargo build --release

# Create app bundle structure
rm -rf YourApp.app
mkdir -p YourApp.app/Contents/{MacOS,Resources}

# Copy binaries
cp target/release/rgbd YourApp.app/Contents/MacOS/
cp target/release/rgb-tray YourApp.app/Contents/MacOS/

# Copy Info.plist
cp Info.plist YourApp.app/Contents/

# Copy icon (if you have one)
# cp icon.icns YourApp.app/Contents/Resources/

# Set executable permissions
chmod +x YourApp.app/Contents/MacOS/*

echo "Build complete!" 