#!/usr/bin/env bash
set -euo pipefail

# Jobs owns the selection; Shared Tooling owns setup and offline admission.
[[ $# == 0 || ( $# == 1 && "$1" == --check ) ]] || {
    echo 'usage: testkit-tool.sh [--check]' >&2; exit 2;
}
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
exec bash "$root/vendor/toko-tooling/scripts/dev/install-rust-tools.sh" --consumer "$root" --package ic-testkit \
    --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli "$@"
