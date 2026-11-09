#!/usr/bin/env bash
# Shared companions: scripts/ci/dependency-pins.jq scripts/ci/cargo-release-group.sh
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
temporary="$(mktemp -d "${TMPDIR:-/tmp}/scheduling-inheritance.XXXXXX")"
trap 'rm -rf -- "$temporary"' EXIT
cd "$root"
# Shared admits the complete product catalog and qualification relationship.
bash scripts/ci/cargo-release-group.sh plan jobs > "$temporary/groups.json"
for workspace in . qualification; do
    cargo metadata --manifest-path "$workspace/Cargo.toml" --no-deps --locked --offline --format-version 1 > "$temporary/members.json"
    yq -p toml -o json '.' "$workspace/Cargo.toml" > "$temporary/root.json"
    if [[ "$workspace" == . ]]; then
        jq '.versions' "$temporary/groups.json" > "$temporary/versions.json"
    else
        # Qualification versions belong to their private packages, not product groups.
        jq -e '. as $m | all(.packages[]; if (.id as $id | $m.workspace_members | index($id)) then .publish == [] else true end)' "$temporary/members.json" >/dev/null
        jq '. as $m | [.packages[] | select(.id as $id | $m.workspace_members | index($id)) | {key:.name,value:.version}] | from_entries' "$temporary/members.json" > "$temporary/versions.json"
    fi
    {
        printf '%s\n' "$root/${workspace#./}/Cargo.toml"
        jq -r '. as $m | .packages[] | select(.id as $id | $m.workspace_members | index($id)) | .manifest_path' "$temporary/members.json"
    } > "$temporary/manifests"
    while IFS= read -r manifest; do
        member=true
        [[ "$manifest" != "$root/${workspace#./}/Cargo.toml" ]] || member=false
        yq -p toml -o json '.' "$manifest" |
            jq -c -L "$root/scripts/ci" --arg file "$manifest" --argjson member "$member" \
                --slurpfile root "$temporary/root.json" --slurpfile versions "$temporary/versions.json" '
                include "dependency-pins";
                {file:$file,data:.} | inheritance_checks($root[0]; $member; $versions[0])' >> "$temporary/findings"
    done < "$temporary/manifests"
done
if [[ -s "$temporary/findings" ]]; then
    jq -r '"\(.file): [\(.rule)] \(.subject): \(.message)"' "$temporary/findings" >&2
    exit 1
fi
echo 'Shared inheritance checks passed with product-group and private qualification version ownership.'
