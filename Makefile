.DEFAULT_GOAL := check

# The shared commands discover qualification/Cargo.toml when its first probe exists.
FORMAT_RUST_ARGS := $(if $(wildcard qualification/Cargo.toml),--extra-workspace qualification)
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

PACKAGE ?= ic-timers

.PHONY: help check lint doc check-calendar-wasm check-executor
help:
	@printf '%s\n' \
	  'Setup: make install-tools; make tools-check' \
	  'make check                    Verify tooling selection and installed snapshot' \
	  'make lint PACKAGE=ic-timers    Lint one package (or ic-jobs), locked and offline' \
	  'make doc PACKAGE=ic-timers     Check one package documentation, locked and offline' \
	  'make check-calendar-wasm       Build/run the calendar Wasm probe (requires Node.js)' \
	  'make check-executor            Installed runtime probe (requires POCKET_IC_BIN)' \
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
