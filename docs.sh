#!/usr/bin/env bash

# Define Variables
LIBS="actus reals time"
SRC="krml"

PROJECT_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

export CARGO_INCREMENTAL=0

bold=$(tput bold)
normal=$(tput sgr0)

# Save current directory.
pushd . >/dev/null

for LIB in ${LIBS}
do
  echo "${bold}Building static doc site for $SRC_$LIB...${normal}"
  cd "$PROJECT_ROOT/$SRC/$LIB"
  # Build docs
  cargo doc --lib
done

# Restore initial directory.
popd >/dev/null
