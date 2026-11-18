#!/usr/bin/env bash
# Scheduling owns the selection; Toko owns installation and offline admission.
set -euo pipefail
case "$#:${1:-}" in
  0:|1:--check) ;;
  *) echo 'usage: metadata-editor.sh [--check]' >&2; exit 2 ;;
esac
exec bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh \
  --consumer "$PWD" --package ic-cargo-metadata --version 0.1.0 \
  --bin ic-cargo-metadata --profile release "$@"
