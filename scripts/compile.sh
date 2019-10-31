#!/usr/bin/env bash

./scripts/build.sh # Build wasm binaries
cargo build # Build native binaries in debug mode
./target/debug/katalchain purge-chain -y --dev # Remove the whole chain data
./target/debug/katalchain --dev # Run chain in dev mode
