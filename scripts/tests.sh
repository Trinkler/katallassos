#!/bin/bash
set -e

PROJECT_ROOT=`git rev-parse --show-toplevel`

cd "$PROJECT_ROOT"

for toml in $(find . -maxdepth 3 -name "Cargo.toml"); do
    cd "${toml::-10}" && cargo test
    cd "$PROJECT_ROOT"
done
