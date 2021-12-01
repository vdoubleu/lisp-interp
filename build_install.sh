#!/bin/bash

echo "Building..."
cargo build --release

echo "Running tests..."
cargo test

echo "Copying binary..."
sudo cp target/release/fl /usr/bin/fl

echo "Copying stdlibs..."
sudo mkdir -p /usr/include/fl
sudo cp -r src/stdlib/* /usr/include/fl/

echo "Successfully built and installed"
