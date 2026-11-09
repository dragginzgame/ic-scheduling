#!/usr/bin/env bash
set -euo pipefail

# Jobs owns the selection; Shared Tooling owns setup and offline admission.
[[ $# == 0 || ( $# == 1 && "$1" == --check ) ]] || {
    echo 'usage: testkit-tool.sh [--check]' >&2; exit 2;
}
exec bash "${BASH_SOURCE[0]%/*}/install-rust-tools.sh" --package ic-testkit \
    --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli "$@"
