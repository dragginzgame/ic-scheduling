.DEFAULT_GOAL := check

TOKO_TOOLING_RELEASE := 1
RELEASE_SETUP_TARGET := release-candidate-setup
RELEASE_PUBLISH_TARGET := publish
RELEASE_DELIVERY_POLICY := ci/release-delivery
CARGO_PUBLISH_REGISTRY := crates-io
CARGO_PUBLISH_VERIFY_TARGETS := publication-admission
CARGO_PUBLISH_ALL_FEATURES := 1
# PocketIC server children otherwise retain the validation logger's pipe after
# the driver exits. Assertions and driver diagnostics still reach the gate log.
export POCKET_IC_MUTE_SERVER := 1
ifeq ($(RELEASE_GROUP),timers)
CARGO_PUBLISH_PACKAGES := ic-timers
else ifeq ($(RELEASE_GROUP),jobs)
CARGO_PUBLISH_PACKAGES := ic-jobs
endif
# Publication is opt-in even when group selection is absent, so bad selections
# fail at admission instead of silently falling back to a package default.
ifneq ($(filter publish publish-check,$(MAKECMDGOALS)),)
ifeq ($(filter timers jobs,$(RELEASE_GROUP)),)
$(error Select RELEASE_GROUP=timers or jobs)
endif
endif
ifneq ($(filter release-patch release-minor release-major,$(MAKECMDGOALS)),)
ifeq ($(strip $(RELEASE_GROUPS)$(RELEASE_GROUP)),)
$(error Select RELEASE_GROUP or an explicit supplier-first RELEASE_GROUPS roster)
endif
endif
ifneq ($(filter release-% publish publish-check,$(MAKECMDGOALS)),)
ifneq ($(strip $(RELEASE_GROUP)),)
ifeq ($(filter timers jobs,$(RELEASE_GROUP)),)
$(error Unknown RELEASE_GROUP)
endif
endif
ifneq ($(strip $(RELEASE_GROUPS)),)
_release_comma := ,
ifneq ($(filter-out timers jobs,$(subst $(_release_comma), ,$(RELEASE_GROUPS))),)
$(error Unknown group in RELEASE_GROUPS)
endif
endif
endif

# The shared commands discover qualification/Cargo.toml when its first probe exists.
FORMAT_RUST_ARGS := $(if $(wildcard qualification/Cargo.toml),--extra-workspace qualification)
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

PACKAGE ?= ic-timers

.PHONY: help check lint doc check-calendar-wasm check-executor check-once check-metrics
help:
	@printf '%s\n' \
	  'Setup: make install-tools; make tools-check' \
	  'make check                    Verify tooling selection and installed snapshot' \
	  'make lint PACKAGE=ic-timers    Lint one package (or ic-jobs), locked and offline' \
	  'make doc PACKAGE=ic-timers     Check one package documentation, locked and offline' \
	  'make check-calendar-wasm       Build/run the calendar Wasm probe (requires Node.js)' \
	  'make check-executor            Installed runtime probe (requires POCKET_IC_BIN)' \
	  'make check-once                Installed Once provider probe (requires POCKET_IC_BIN)' \
	  'make check-metrics             Compare metrics off/on Wasm costs (requires POCKET_IC_BIN)' \
	  'make fmt / fmt-check          Shared Rust and manifest formatting' \
	  'make deps-list                List local package versions in maintained workspaces' \
	  'make deps-update              Update maintained workspace locks using cargo update -v' \
	  'make deps-update DEPENDENCY_PACKAGE=toko-tooling  Update only the selected supplier' \
	  'make toko-tooling-preview     Preview the locked tooling snapshot' \
	  'make toko-tooling-update      Install the locked tooling snapshot and tools'

check: toko-tooling-check

lint doc: toko-tooling-check | toko-execution-check
lint:
	$(_toko_recursive_prefix)cargo clippy -p '$(PACKAGE)' --all-targets --all-features --locked --offline -- -D warnings

doc:
	$(_toko_recursive_prefix)RUSTDOCFLAGS='-D warnings' cargo doc -p '$(PACKAGE)' --all-features --no-deps --locked --offline

check-calendar-wasm: toko-tooling-check | toko-execution-check
	$(_toko_recursive_prefix)cargo build --manifest-path qualification/Cargo.toml -p calendar-probe --target-dir qualification/target --release --target wasm32-unknown-unknown --locked --offline
	node qualification/calendar-probe/check.mjs qualification/target/wasm32-unknown-unknown/release/calendar_probe.wasm

# The host server is prepared separately; no implicit download in a check.
check-executor: toko-tooling-check | toko-execution-check
	@test -n "$(POCKET_IC_BIN)" || { echo "Set POCKET_IC_BIN to a prepared PocketIC 16.0.0 server" >&2; exit 1; }
	$(_toko_recursive_prefix)cargo build --manifest-path qualification/Cargo.toml -p executor-probe --target-dir qualification/target --release --target wasm32-unknown-unknown --locked --offline
	$(_toko_recursive_prefix)POCKET_IC_BIN="$(POCKET_IC_BIN)" cargo run --manifest-path qualification/Cargo.toml -p executor-driver --target-dir qualification/target --locked --offline -- qualification/target/wasm32-unknown-unknown/release/executor_probe.wasm

check-once: toko-tooling-check | toko-execution-check
	@test -n "$(POCKET_IC_BIN)" || { echo "Set POCKET_IC_BIN to a prepared PocketIC 16.0.0 server" >&2; exit 1; }
	$(_toko_recursive_prefix)cargo build --manifest-path qualification/Cargo.toml -p once-probe --target-dir qualification/target --release --target wasm32-unknown-unknown --locked --offline
	$(_toko_recursive_prefix)POCKET_IC_BIN="$(POCKET_IC_BIN)" cargo run --manifest-path qualification/Cargo.toml -p once-driver --target-dir qualification/target --locked --offline -- qualification/target/wasm32-unknown-unknown/release/once_probe.wasm

check-metrics: toko-tooling-check | toko-execution-check
	@test -n "$(POCKET_IC_BIN)" || { echo "Set POCKET_IC_BIN to a prepared PocketIC 16.0.0 server" >&2; exit 1; }
	$(_toko_recursive_prefix)cargo build --manifest-path qualification/Cargo.toml -p metrics-probe --no-default-features --target-dir qualification/target --release --target wasm32-unknown-unknown --locked --offline
	cp qualification/target/wasm32-unknown-unknown/release/metrics_probe.wasm qualification/target/metrics_off.wasm
	$(_toko_recursive_prefix)cargo build --manifest-path qualification/Cargo.toml -p metrics-probe --features metrics --target-dir qualification/target --release --target wasm32-unknown-unknown --locked --offline
	cp qualification/target/wasm32-unknown-unknown/release/metrics_probe.wasm qualification/target/metrics_on.wasm
	$(_toko_recursive_prefix)POCKET_IC_BIN="$(POCKET_IC_BIN)" cargo run --manifest-path qualification/Cargo.toml -p once-driver --target-dir qualification/target --locked --offline -- --metrics-cost qualification/target/metrics_off.wasm qualification/target/metrics_on.wasm

.PHONY: check-formatting-hooks qualify qualify-rust qualify-archives qualify-entropy
check-formatting-hooks: toko-tooling-check | toko-execution-check
	node qualification/formatting-check.mjs

# Maintainer waived Entropy integration for the selected deliveries (2026-11-15/16).
# Ordinary qualification and subsequent versions retain the consumer journey.
QUALIFICATION_ENTROPY_TARGET := $(if $(filter timers:0.45.10 jobs:0.38.0 timers:0.45.11 jobs:0.38.1,$(RELEASE_GROUP):$(RELEASE_VERSION)),,qualify-entropy)
qualify:
	$(_toko_recursive_prefix)VALIDATION_REPOSITORY_ROOT="$(CURDIR)" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --fail-fast tools-check check fmt-check check-formatting-hooks qualify-rust check-calendar-wasm check-executor check-once check-metrics $(QUALIFICATION_ENTROPY_TARGET) qualify-archives
qualify-rust:
	bash ci/qualify.sh rust
qualify-archives:
	bash ci/qualify.sh archives
qualify-entropy:
	bash ci/qualify.sh entropy

.PHONY: release-candidate-setup release-version release-files release-preflight release-verify release-prepare-version release-prepared-check release-commit-check release-committed-check release-tagged-check release-push-check publication-admission
release-candidate-setup:
	@bash ci/release.sh setup
release-version:
	@bash ci/release.sh version
release-files:
	@bash ci/release.sh files
release-preflight:
	@bash ci/release.sh preflight
release-verify:
	@bash ci/release.sh verify
release-prepare-version:
	@bash ci/release.sh prepare
release-prepared-check:
	@bash ci/release.sh prepared
release-commit-check:
	@bash ci/release.sh index
release-committed-check release-tagged-check release-push-check:
	@bash ci/release.sh committed
publication-admission:
	@bash ci/release.sh publication
