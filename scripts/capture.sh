#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release --locked
for preset in cinematic edge-on top-down jets; do
  target/release/singularity-rust --preset "$preset" --quality high \
    --time 24 --no-ui --width 1600 --height 1000 \
    --capture "docs/screenshots/$preset.png"
done
