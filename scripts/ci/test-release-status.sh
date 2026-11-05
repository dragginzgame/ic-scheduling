#!/usr/bin/env bash
set -euo pipefail
# The fixture owns group/commit selection independently of its release caller.
unset RELEASE_GROUP RELEASE_COMMIT
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
scratch_parent="${TMPDIR:-$root/target/tmp}"
case "$scratch_parent" in *$'\r'*|*$'\n'*) echo 'scratch directory contains CR/LF' >&2; exit 2 ;; esac
[[ -n "${TMPDIR:-}" ]] || mkdir -p "$scratch_parent"
fixture="$(fixture_create "$scratch_parent/scheduling-release-status.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    trap - EXIT
    fixture_completion_status "$status" "$fixture_complete" || status=$?
    printf 'Readiness evidence retained: %s\n' "$fixture" >&2
    exit "$status"
}
trap finish EXIT
mkdir -p "$fixture/repo/scripts/release" "$fixture/repo/scripts/ci" "$fixture/repo/ci" "$fixture/bin"
cp "$root/scripts/release/status.pl" "$fixture/repo/scripts/release/"
cp "$root/scripts/ci/StableVersion.pm" "$fixture/repo/scripts/ci/"
cat > "$fixture/repo/Cargo.toml" <<'TOML'
[workspace.package]
repository = "https://github.com/dragginzgame/ic-scheduling"
TOML
printf '{"groups":[{"name":"jobs"},{"name":"timers"}]}\n' > "$fixture/repo/ci/cargo-release-groups.json"
cat > "$fixture/repo/scripts/release/group.sh" <<'GROUP'
#!/usr/bin/env bash
[[ "$1" == plan ]] || exit 91
if [[ "${STATUS_GROUP_FAILURE:-0}" == 1 && -z "${RELEASE_COMMIT:-}" ]]; then exit 53; fi
case "$RELEASE_GROUP" in jobs) current=1.2.3 ;; timers) current=4.5.6 ;; *) exit 92 ;; esac
printf '{"primary":"ic-%s","previous":"%s","changelog":"releases/%s/CHANGELOG.md"}\n' "$RELEASE_GROUP" "$current" "$RELEASE_GROUP"
GROUP
for selection in 'jobs 1.2.4' 'timers 4.5.7'; do
    read -r group proposed <<< "$selection"
    mkdir -p "$fixture/repo/releases/$group"
    printf '## [%s] - Unreleased\n' "$proposed" > "$fixture/repo/releases/$group/CHANGELOG.md"
done
cat > "$fixture/bin/curl" <<'CURL'
#!/usr/bin/env bash
printf 'request\n' >> "$STATUS_NETWORK_EVENTS"
url="${@: -1}"
case "$url" in
    *api.github.com*)
        [[ "${STATUS_CI_FAILURE:-0}" != 1 ]] || exit 28
        printf '{"workflow_runs":[{"name":"CI","head_sha":"%s","status":"completed","conclusion":"success","html_url":"https://example.invalid/ci"}]}\n' "$STATUS_FIXTURE_HEAD" ;;
    *crates.io/api/v1/crates/ic-jobs)
        [[ "${STATUS_REGISTRY_FAILURE:-0}" != 1 ]] || exit 22
        printf '{"crate":{"id":"ic-jobs","max_stable_version":"1.2.2"}}\n' ;;
    *crates.io/api/v1/crates/ic-timers)
        [[ "${STATUS_REGISTRY_FAILURE:-0}" != 1 ]] || exit 22
        printf '{"crate":{"id":"ic-timers","max_stable_version":"4.5.6"}}\n' ;;
    *) exit 93 ;;
esac
CURL
chmod +x "$fixture/bin/curl"
export PATH="$fixture/bin:$PATH" STATUS_NETWORK_EVENTS="$fixture/network-events"
export STATUS_OFFLINE=0 STATUS_JSON=0
cd "$fixture/repo"
git init -q
git config user.name 'Readiness fixture'
git config user.email 'fixture@example.invalid'
git config commit.gpgsign false
git config tag.gpgsign false
git config core.hooksPath /dev/null
git add -- .
git commit -qm 'Fixture source'
git tag -a jobs-v1.2.3 -m 'Jobs fixture'
git tag -a timers-v4.5.6 -m 'Timers fixture'
git init --bare -q "$fixture/remote.git"
git remote add origin "$fixture/remote.git"
git push -q --atomic origin HEAD:main refs/tags/jobs-v1.2.3 refs/tags/timers-v4.5.6
STATUS_FIXTURE_HEAD="$(git rev-parse HEAD)"
export STATUS_FIXTURE_HEAD
git for-each-ref --format='%(refname) %(objectname)' > "$fixture/refs-before"
git --git-dir="$fixture/remote.git" for-each-ref --format='%(refname) %(objectname)' > "$fixture/remote-before"
git status --porcelain=v1 > "$fixture/status-before"
: > "$STATUS_NETWORK_EVENTS"
perl scripts/release/status.pl --offline --json > "$fixture/offline.json"
jq -e '.source.clean and .source.ci.state == "unknown" and all(.groups[]; .published == null and .tag.remote == "unknown")' "$fixture/offline.json"
[[ ! -s "$STATUS_NETWORK_EVENTS" ]]
perl scripts/release/status.pl --json > "$fixture/online.json"
jq -e '.source.ci.conclusion == "success" and
    .groups[0].current == "1.2.3" and .groups[0].published == "1.2.2" and .groups[0].proposed == "1.2.4" and
    .groups[1].published == "4.5.6" and all(.groups[]; .tag.remote == "matches") and
    (.release_gate | contains("complete gate"))' "$fixture/online.json"
STATUS_REGISTRY_FAILURE=1 STATUS_CI_FAILURE=1 perl scripts/release/status.pl --json > "$fixture/unknown.json"
jq -e '.source.ci.state == "unknown" and all(.groups[]; .published == null) and (.blockers | length > 0)' "$fixture/unknown.json"
STATUS_GROUP_FAILURE=1 perl scripts/release/status.pl --offline --json > "$fixture/unprepared.json"
jq -e 'all(.groups[]; .current_source == "committed") and (.blockers | length >= 2)' "$fixture/unprepared.json"
printf 'uncommitted\n' > source-change
mkdir -p .git/release-state/lock
RELEASE_GROUP=timers perl scripts/release/status.pl --offline --json > "$fixture/blocked.json"
jq -e '.source.clean == false and (.groups | length == 1) and .groups[0].group == "timers" and (.blockers | length >= 2)' "$fixture/blocked.json"
rm source-change
rmdir .git/release-state/lock
: > "$STATUS_NETWORK_EVENTS"
if RELEASE_GROUP=unknown perl scripts/release/status.pl --json > "$fixture/refused.log" 2>&1; then exit 1; fi
[[ ! -s "$STATUS_NETWORK_EVENTS" ]]
git for-each-ref --format='%(refname) %(objectname)' > "$fixture/refs-after"
git --git-dir="$fixture/remote.git" for-each-ref --format='%(refname) %(objectname)' > "$fixture/remote-after"
git status --porcelain=v1 > "$fixture/status-after"
cmp "$fixture/refs-before" "$fixture/refs-after"
cmp "$fixture/remote-before" "$fixture/remote-after"
cmp "$fixture/status-before" "$fixture/status-after"
fixture_complete=true
echo 'Readiness reports published/proposed gaps, unavailable observations and blocked inputs without source/ref/remote writes (real Git; substituted registry/CI/group metadata).'
