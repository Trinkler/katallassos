#!/usr/bin/env bash

set -e

PROJECT_ROOT=`git rev-parse --show-toplevel`

cd "$PROJECT_ROOT"

# Save current directory.
pushd . >/dev/null

for SRC in node/runtime/wasm
do
  echo "Building webassembly binary in $SRC..."
  cd "$SRC"
  ./build.sh "$@"

  cd - >> /dev/null
done

# Restore initial directory.
popd >/dev/null
