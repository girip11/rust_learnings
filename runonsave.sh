#!/usr/bin/env bash

set -eu

rustfmt --emit=stdout --quiet --unstable-features \
  --skip-children "$1"

clippy-driver --edition 2021 -Cpanic=abort "$1"

rustc "$1" --out-dir "$2/tryouts/rust_handbook_chapters/.build"
