#!/usr/bin/env bash
# Shared companions: vendor/toko-tooling/scripts/dev/tooling-lock.pl ci/workspace-manifests.txt
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
product="$(perl "$root/vendor/toko-tooling/scripts/dev/tooling-lock.pl" "$root")"
qualification="$(perl "$root/vendor/toko-tooling/scripts/dev/tooling-lock.pl" "$root/qualification")"
[[ "$product" == "$qualification" ]] || {
    echo 'product and qualification must select the same authenticated Tooling release' >&2
    exit 1
}
# The explicit inventory feeds source/format/release inputs; Cargo owns membership.
observed="$(for workspace in "$root" "$root/qualification"; do
    cargo metadata --manifest-path "$workspace/Cargo.toml" --no-deps --locked --offline --format-version 1 |
        jq -er --arg root "$root/" '. as $m | .packages[] | select(.id as $id | $m.workspace_members | index($id)) | .manifest_path | ltrimstr($root)' || exit
done | LC_ALL=C sort)"
[[ "$observed" == "$(LC_ALL=C sort "$root/ci/workspace-manifests.txt")" ]] || {
    echo 'workspace manifest inventory disagrees with Cargo membership' >&2
    exit 1
}
echo 'Product and qualification Tooling selections and member inventories agree.'
