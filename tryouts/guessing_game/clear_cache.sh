#!/usr/bin/env bash

set -eu

# Run this when we hit Blocking waiting for file lock from cargo
rm -rf "$HOME/.cargo/registry/index/*"
rm -fr "$HOME/.cargo/.package-cache"
