#!/usr/bin/env bash
set -euo pipefail

# One compiler policy for builds, launchers and CI setup.
# This helper never installs a toolchain or resolves dependencies.
[[ $# -ge 1 ]] || { echo 'usage: minimum-rust.sh --print|--check|cargo|rustc [ARGS...]' >&2; exit 2; }
case "$1" in
    --print|--check) [[ $# == 1 ]] || exit 2 ;;
    cargo|rustc) ;;
    *) echo 'expected --print, --check, cargo or rustc' >&2; exit 2 ;;
esac
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
version="$(bash "$root/scripts/ci/read-cargo-workspace-rust-version.sh" "$root/Cargo.toml")"
[[ "$version" == *.*.* ]] || version="$version.0"
[[ "$version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || exit 2
pin="$("${YQ:-yq}" -p toml -o json '.' "$root/rust-toolchain.toml" | jq -ers '
    if length == 1 then .[0].toolchain.channel | select(type == "string") else empty end
')"
[[ "$pin" == "$version" ]] || {
    echo "Rust policy mismatch: workspace floor $version, pinned compiler $pin" >&2
    exit 1
}
export RUSTUP_AUTO_INSTALL=0
if [[ "$1" == --print ]]; then printf '%s\n' "$version"; exit 0; fi
if [[ "$1" == --check ]]; then
    for tool in rustc cargo; do
        identity="$("$tool" --version --verbose)"
        actual="$(printf '%s\n' "$identity" | sed -n 's/^release: //p')"
        [[ "$actual" == "$version" ]] || {
            echo "Rust policy mismatch: $tool reports $actual, required $version" >&2
            exit 1
        }
        printf '%s\n' "$identity"
    done
    exit 0
fi
tool="$1"
shift
exec "$tool" "+$version" "$@"
