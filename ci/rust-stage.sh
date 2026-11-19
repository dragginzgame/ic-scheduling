#!/usr/bin/env bash
# Thin Scheduling policy over Toko's final-candidate and exact-stage mechanics.
set -euo pipefail
root="$PWD"
shared="$root/vendor/toko-tooling/scripts/ci"
export PATH="$root/.tools/host/bin:$root/.tools/rust/common/bin:$PATH"
scope="${QUALIFICATION_SCOPE:-workspace}"
pointer="$root/target/release-validation/rust-${RELEASE_GROUP:-none}-${RELEASE_SOURCE:-none}.path"
if [[ "${1:-}" == check ]]; then
  [[ -f "$pointer" && ! -L "$pointer" ]] || exit 1
  attempt="$(cat "$pointer")"
  if [[ "$attempt" == ordinary ]]; then echo ordinary; exit 0; fi
  case "$attempt" in "$root"/target/release-validation/rust-attempt.*) ;; *) exit 1 ;; esac
  [[ -d "$attempt" && ! -L "$attempt" ]] || exit 1
  perl "$shared/prepare-cargo-stage-source.pl" --verify-projection "$attempt/projection.json" >&2
  identity="$(jq -er .identity "$attempt/projection.json")"
  perl "$shared/seal-validation-stage.pl" "$attempt/admission.json" "${RELEASE_SOURCE:?}" "${RELEASE_GROUP:?}" "$identity"
  exit
fi
[[ $# == 0 ]] || exit 2
ordinary() {
  echo "Rust stage executes on current source: $1" >&2
  bash ci/qualify.sh rust "$scope"
  if [[ -n "${RELEASE_SOURCE:-}" ]]; then
    mkdir -p "${pointer%/*}"
    [[ ! -L "$pointer" ]] || exit 1
    printf '%s\n' ordinary > "$pointer"
  fi
  exit
}
[[ -n "${RELEASE_SOURCE:-}" ]] || ordinary 'development or repository CI'
[[ "$scope" == workspace ]] || ordinary 'narrow Jobs mapping'
[[ "$(uname -s)" == Linux ]] || ordinary 'candidate reuse is qualified on Linux only'
# Do not erase ambient overrides to manufacture a cache hit.
while IFS= read -r name; do
  case "$name" in
    CARGO_HOME|CARGO_TARGET_DIR|RUSTFLAGS|RUSTDOCFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER|CARGO_BUILD_*|CARGO_TARGET_*|RUSTUP_TOOLCHAIN)
      [[ -z "${!name}" ]] || ordinary "custom $name" ;;
  esac
done < <(compgen -e)
directory="$root"
while :; do
  [[ ! -e "$directory/.cargo/config" && ! -e "$directory/.cargo/config.toml" ]] || ordinary 'ancestor Cargo configuration'
  [[ "$directory" != / ]] || break
  directory="${directory%/*}"; directory="${directory:-/}"
done
[[ ! -e "$HOME/.cargo/config" && ! -e "$HOME/.cargo/config.toml" ]] || ordinary 'Cargo home configuration'
mkdir -p target/release-validation target/release-validation/rust-tmp
attempt="$(mktemp -d "$root/target/release-validation/rust-attempt.XXXXXX")"
editor="$(bash ci/metadata-editor.sh --check)"
perl "$shared/prepare-cargo-stage-source.pl" --root "$root" --state target/release-validation/rust-sources \
  --source "$RELEASE_SOURCE" --editor "$editor" > "$attempt/projection.json"
identity="$(jq -er .identity "$attempt/projection.json")"
candidate="$(jq -er .directory "$attempt/projection.json")"
tools=()
for tool in bash cargo cc c++ ld ar make perl git jq yq tar awk sed grep sort head tail xargs cp mv mkdir rm rmdir mktemp tee shasum; do
  tools+=("$(command -v "$tool")")
done
jq -cnS --args '$ARGS.positional | {files:.}' "${tools[@]}" "$editor" > target/release-validation/rust-tools.json
jq -cnS --arg path "$PATH" --arg home "$HOME" --arg tmp "$root/target/release-validation/rust-tmp" \
  --arg tools "$root/target/release-validation/rust-tools.json" --arg identity "$identity" \
  '{PATH:$path,HOME:$home,TMPDIR:$tmp,LC_ALL:"C",RUSTUP_AUTO_INSTALL:"0",CARGO_STAGE_TOOLS:$tools,CARGO_STAGE_IDENTITY:$identity}' \
  > target/release-validation/rust-environment.json
(
  cd "$candidate"
  perl "$shared/run-validation-stage.pl" --state "$root/target/release-validation/rust-proofs" \
    --stage scheduling-rust --contract scheduling-rust/1 --source "$RELEASE_SOURCE" --group "$RELEASE_GROUP" \
    --observe "$shared/observe-cargo-stage.pl" --environment "$root/target/release-validation/rust-environment.json" \
    --max-bytes 536870912 --admission-output "$attempt/admission.json" -- bash ci/qualify.sh rust workspace
)
perl "$shared/seal-validation-stage.pl" "$attempt/admission.json" "$RELEASE_SOURCE" "$RELEASE_GROUP" "$identity" > "$attempt/sealed.json"
[[ ! -L "$pointer" ]] || exit 1
printf '%s\n' "$attempt" > "$pointer"
jq -r '"Rust stage: " + .admission.mode + "; original source=" + .origin.source' "$attempt/sealed.json"
