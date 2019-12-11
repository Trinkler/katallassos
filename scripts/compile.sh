#!/usr/bin/env bash

./scripts/init.sh # Build wasm binaries
cargo build # Build native binaries in debug mode
#./target/debug/katalchain purge-chain -y --dev 
# TODO: Fix local use only
rm -rf /home/x4/.local/share/katalchain/chains/dev # Remove the whole chain data
./target/debug/katalchain --dev # Run chain in dev mode
