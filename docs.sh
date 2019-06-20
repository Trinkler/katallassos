#!/usr/bin/env bash

# Define Variables
PROJECT_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

bold=$(tput bold)
normal=$(tput sgr0)

echo "${bold}Building static doc site...${normal}"
# Build docs
cargo doc
# Move to docs folder to deploy
mkdir docs
cp -r target/doc/* docs
