.DEFAULT_GOAL := help
export RELEASE_DELIVERY ?= direct
export RELEASE_GROUP
# Refuse absent, empty or unknown selections before prerequisites or setup run.
ifneq ($(filter release-patch release-minor release-major release-resume publish publish-check package,$(MAKECMDGOALS)),)
ifneq ($(words $(RELEASE_GROUP)),1)
$(error Select RELEASE_GROUP=jobs or timers explicitly)
endif
ifeq ($(filter jobs timers,$(RELEASE_GROUP)),)
$(error Select RELEASE_GROUP=jobs or timers explicitly)
endif
ifneq ($(strip $(PACKAGE)),)
ifneq ($(PACKAGE),ic-$(RELEASE_GROUP))
$(error PACKAGE must belong to the explicitly selected RELEASE_GROUP)
endif
endif
endif
override SHARED_TOOLING_ROOT := $(CURDIR)
override RELEASE_DELIVERY_POLICY := $(CURDIR)/ci/release-delivery.txt
# Optimized qualification uses Binaryen; no other IC executable is required.
IC_TOOLS := wasm-opt
LOCAL_TOOL_INSTALL_TARGETS := install-ic-tools
LOCAL_TOOL_CHECK_TARGETS := ic-tools-check
export PATH := $(CURDIR)/.tools/ic/bin:$(PATH)
include make/consumer.mk
TESTKIT_TOOL = bash scripts/dev/testkit-tool.sh
MINIMUM_RUST = bash scripts/dev/minimum-rust.sh
RELEASE_METADATA_LOCKFILE ?= $(CURDIR)/Cargo.lock
RELEASE_METADATA_TOOL = bash scripts/dev/install-rust-tools.sh --package ic-build --lockfile "$(RELEASE_METADATA_LOCKFILE)" --bin ic-build-release-metadata --profile debug --features cli
VALIDATION_INPUTS := --input Cargo.toml --input Cargo.lock --input rust-toolchain.toml \
	--input .shared-tooling.snapshot --input Makefile --input scripts/dev/testkit-tool.sh --input scripts/dev/queue-command.sh --input scripts/dev/minimum-rust.sh --input ci/tool-versions.env \
	--input ci/ic-tools.tsv --input ci/release-delivery.txt --input ci/cargo-dependency-boundaries.json --input ci/cargo-release-groups.json \
	--input .github/workflows/ci.yml --input .github/workflows/recovery.yml --input ci/docs-only-paths.txt --input scripts/ci/classify-ci-change.pl \
	$(foreach manifest,$(wildcard crates/*/Cargo.toml apps/*/Cargo.toml),--input $(manifest))

.PHONY: help version install-hooks check test-jobs test-consumers check-msrv check-wasm clippy docs-check check-doc-links shared-tooling-check check-pins check-release-commands test-release-tooling test-formatting-evidence package publish-check publish ci
help:
	@echo "IC Scheduling: shared workspace; explicitly select RELEASE_GROUP=jobs or timers"
	@echo "Timers: test-timers, test-timer-tooling, check-timer-provider, check-timer-dependencies, check-timer-probes"
	@echo "Explicit Timers IC qualification: test-timer-watchdog, test-timer-cohorts"
	@echo "Focused selections: see README (Consumer qualification) for package/test/target commands"
	@echo "Aggregates: check, test-jobs, test-consumers, check-msrv, check-wasm, clippy, docs-check, fmt-check"
	@echo "Consumer: check-consumer (native, Wasm, harness compilation); build-consumer (Wasm artifact)"
	@echo "Explicit IC qualification: install-testkit-tools; install-consumer-server; test-canister[-optimized]"
	@echo "Local canister queue: demo (interactive); test-demo (live scenario); test-demo-cli (process shutdown)"
	@echo "Metadata: shared-tooling-check, check-doc-links, check-pins, check-dependency-boundaries, check-dependency-graphs, check-release-commands"
	@echo "Read-only readiness: release-status (STATUS_OFFLINE=1, STATUS_JSON=1; optional RELEASE_GROUP)"
	@echo "Tooling fixtures: version, test-release-tooling, test-formatting-evidence, test-consumer-tooling"
	@echo "Release fixture groups: test-release-tooling-{retention,delivery,recovery,metadata,publication}"
	@echo "Hook qualification: formatting-hook-check (disposable Git copies; prepared formatters required)"
	@echo "Registry preparation: package (offline), publish-check (registry dry run; no upload)"
	@echo "Explicit setup: install-tools, tools-check, install-hooks"
	@echo "Release editor: install-release-tools; release-tools-check (offline-admitted path)"
	@echo "Complete gate: ci (explicit request or configured CI)"
	@echo "Authorized release: release-patch, release-minor, release-major (requires a configured remote)"
	@echo "Authorized publication: publish (clean HEAD, matching pushed annotated tag; crates.io)"
	@echo "Recovery: release-resume VERSION=X.Y.Z (normal release targets start fresh)"

check:
	+cargo check -p ic-jobs --all-targets --all-features --locked --offline

test-jobs:
	+cargo test -p ic-jobs --tests --all-features --locked --offline

test-consumers:
	+$(MINIMUM_RUST) cargo test -p ic-jobs-test-consumer --test composition --test stored_wire --locked --offline
	+$(MINIMUM_RUST) cargo run -p ic-jobs --example persisted_job --locked --offline

.PHONY: check-consumer build-consumer install-testkit-tools testkit-tools-check install-consumer-server consumer-server-check test-canister test-canister-optimized
check-consumer:
	+$(MINIMUM_RUST) cargo test -p ic-jobs-test-consumer --lib --test api_wire --locked --offline
	+$(MAKE) --no-print-directory build-consumer
	+$(MINIMUM_RUST) cargo test -p ic-jobs-canister-tests --bin jobs-capture-artifact --example queue --test queue_cli --test recovery --locked --offline

build-consumer:
	+$(MINIMUM_RUST) cargo build -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline

.PHONY: build-recovery-tools
build-recovery-tools:
	+$(MINIMUM_RUST) cargo build -p ic-jobs-canister-tests --bin jobs-capture-artifact --locked --offline

install-testkit-tools:
	+$(TESTKIT_TOOL)

testkit-tools-check:
	+$(TESTKIT_TOOL) --check

install-consumer-server:
	+@cli="$$( $(TESTKIT_TOOL) --check )" && "$$cli" setup

consumer-server-check:
	+@cli="$$( $(TESTKIT_TOOL) --check )" && "$$cli" check

# Only this explicitly selected target launches PocketIC. Admission precedes
# artifact compilation; normal CI compiles the harness without executing it.
test-canister test-canister-optimized: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+$(MAKE) --no-print-directory build-recovery-tools
	+$(MINIMUM_RUST) cargo test -p ic-jobs-canister-tests --test recovery --locked --offline --no-run
	+bash scripts/ci/test-canister.sh $(if $(filter test-canister-optimized,$@),--optimized)

.PHONY: demo test-demo test-demo-cli
demo: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+$(MINIMUM_RUST) cargo build -p ic-jobs-canister-tests --example queue --locked --offline
	+bash scripts/dev/queue-command.sh demo

test-demo: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+$(MINIMUM_RUST) cargo test -p ic-jobs-canister-tests --example queue --locked --offline --no-run
	+bash scripts/dev/queue-command.sh test-demo

# Spawn-mode process tests intentionally do not borrow the batch runner's server.
test-demo-cli: consumer-server-check
	+$(MAKE) --no-print-directory build-consumer
	+$(MINIMUM_RUST) cargo build -p ic-jobs-canister-tests --example queue --locked --offline
	+bash scripts/dev/queue-command.sh test-demo-cli

check-msrv:
	+$(MINIMUM_RUST) cargo check -p ic-jobs --lib --locked --offline
	+$(MINIMUM_RUST) cargo check -p ic-jobs-test-consumer --test composition --test stored_wire --locked --offline

check-wasm:
	+$(MINIMUM_RUST) cargo check -p ic-jobs --lib --target wasm32-unknown-unknown --locked --offline
	+$(MINIMUM_RUST) cargo check -p ic-jobs-test-consumer --lib --target wasm32-unknown-unknown --locked --offline

clippy:
	+cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings
	+cargo clippy -p ic-jobs-test-consumer -p ic-jobs-canister-test-support -p ic-jobs-canister-tests -p ic-timers --all-targets --locked --offline -- -D warnings
	+cargo clippy -p ic-jobs-test-consumer --target wasm32-unknown-unknown --locked --offline -- -D warnings

docs-check:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-jobs -p ic-timers --all-features --locked --offline --no-deps
	+cargo test -p ic-jobs --doc --all-features --locked --offline

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root . *.md releases/history.md releases/*/CHANGELOG.md docs/*.md docs/changelog/*.md docs/principles/*.md docs/status/*.md rules/*.md audits/*.md tasks/*.md $(shell rg --files docs/timers -g '*.md') crates/ic-timers/SAFETY.md crates/ic-timers/CHANGELOG.md $(wildcard apps/*/README.md apps/*/tests/fixtures/README.md crates/*/README.md crates/*/tests/fixtures/README.md)

check-pins:
	+bash scripts/ci/check-dependency-pins.sh --cargo-inheritance

.PHONY: check-dependency-boundaries check-release-groups package-all
check-release-groups:
	+@set -e; metadata="$$(mktemp)"; trap 'rm -f "$$metadata"' EXIT; \
		yq -p toml -o json '.' Cargo.toml > "$$metadata"; \
		jq -e --slurpfile root "$$metadata" '($$root[0].workspace.package | has("version") | not) and all(.groups[]; .version == "package" and .changelog == ("releases/" + .name + "/CHANGELOG.md"))' ci/cargo-release-groups.json > /dev/null
	+bash scripts/ci/cargo-release-group.sh plan jobs > /dev/null
	+bash scripts/ci/cargo-release-group.sh plan timers > /dev/null

check-dependency-boundaries:
	+bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)" --external-ic

# Jobs owns these concrete exclusions; Shared owns observation and matching.
CORE_GRAPH_FORBIDDEN := clap ctrlc candid candid_derive candid_parser ic-cdk ic-cdk-macros \
	ic-cdk-executor ic-cdk-timers ic-platform ic-platform-runtime ic-platform-types ic-platform-macros ic-platform-runtime-guard ic0 ic-memory ic-stable-structures ic-storage \
	ic-timers ic-metrics ic-logging ic-wire ic-wire-derive postcard ic-build ic-host-fs ic-host-process ic-testkit \
	ic-testkit-candid ic-testkit-native ic-jobs-test-consumer ic-jobs-canister-test-support pocket-ic serde_json serde_test
QUALIFICATION_GRAPH_FORBIDDEN := ic-cdk-executor ic0 clap ctrlc serde_json ic-storage candid candid_derive candid_parser ic-cdk ic-agent ic-build ic-host-fs \
	ic-host-process ic-host-artifacts ic-host-text ic-testkit ic-testkit-candid ic-testkit-native ic-jobs-canister-test-support pocket-ic tokio
# Jobs and Timers share one Runtime owner; Memory keeps its storage facade.
# Keep one executor, raw-system provider and entrypoint generator, without
# application Principal conversion or hashing in the deployed graph.
.PHONY: check-dependency-graphs
check-dependency-graphs:
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target wasm32-unknown-unknown --all-features \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs-test-consumer \
		--target wasm32-unknown-unknown --all-features \
		--require ic-timers=1 --require ic-platform-runtime-guard=1 \
		--require ic-platform-runtime=1 --require ic-platform-types=1 --require ic-platform-macros=1 \
		--forbid sha2 --forbid-feature ic-platform/principal-external \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs-test-consumer \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid ic-cdk --forbid ic-platform --forbid ic-platform-runtime --forbid ic-platform-macros --forbid ic-platform-runtime-guard --forbid-feature ic-platform/cdk \
		--forbid ic-jobs --forbid ic-memory --forbid ic-timers --forbid serde_json \
		--forbid ic-stable-structures --forbid ic-cdk-executor --forbid ic0 --forbid ic-metrics --forbid ic-logging \
		--forbid ic_principal --forbid sha2 \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs-canister-tests \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid candid --forbid candid_derive --forbid candid_parser \
		--forbid pocket-ic --forbid ic-testkit --forbid ic-jobs-canister-test-support --forbid ctrlc --forbid-feature ic-platform/candid \
		--forbid-feature ic-platform/pocket-ic

check-release-commands:
	RELEASE_GROUP=jobs bash scripts/ci/check-release-commands.sh . make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh scripts/ci/check-release-delivery.sh ci/release-delivery.txt

test-release-tooling: format-tools-check
	+@editor="$$( $(RELEASE_METADATA_TOOL) --check )" && \
		CARGO_RELEASE_METADATA_EDITOR="$$editor" CARGO_RELEASE_METADATA_MAX_BYTES=1048576 \
		bash scripts/release/test-tooling.sh

RELEASE_TOOLING_GROUPS := delivery recovery metadata publication
.PHONY: test-release-tooling-retention $(addprefix test-release-tooling-,$(RELEASE_TOOLING_GROUPS))
test-release-tooling-retention:
	+bash scripts/release/test-tooling.sh retention

$(addprefix test-release-tooling-,$(RELEASE_TOOLING_GROUPS)): format-tools-check
	+@editor="$$( $(RELEASE_METADATA_TOOL) --check )" && \
		CARGO_RELEASE_METADATA_EDITOR="$$editor" CARGO_RELEASE_METADATA_MAX_BYTES=1048576 \
		bash scripts/release/test-tooling.sh $(@:test-release-tooling-%=%)

.PHONY: install-release-tools release-tools-check
install-release-tools:
	+$(RELEASE_METADATA_TOOL)

release-tools-check:
	+@$(RELEASE_METADATA_TOOL) --check

test-formatting-evidence:
	+bash scripts/ci/test-formatting-evidence.sh

.PHONY: test-consumer-tooling
test-consumer-tooling: build-recovery-tools
	+bash scripts/ci/test-consumer-tooling.sh

.PHONY: test-ci-change-selection
test-ci-change-selection: host-tools-check
	+bash scripts/ci/test-ci-change-selection.sh

.PHONY: test-recovery-evidence
test-recovery-evidence: host-tools-check
	+bash scripts/ci/test-recovery-evidence.sh

.PHONY: test-release-status
test-release-status: host-tools-check
	+bash scripts/ci/test-release-status.sh

.PHONY: formatting-hook-check
formatting-hook-check: format-tools-check
	+bash scripts/ci/check-jobs-formatting.sh

package-all:
	+cargo package -p ic-jobs -p ic-timers --all-features --locked --offline --allow-dirty

package:
	+@package="$$(bash scripts/release/group.sh package)" && cargo package -p "$$package" --all-features --locked --offline --allow-dirty

publish-check:
	+$(MAKE) --no-print-directory clippy
	+@package="$$(bash scripts/release/group.sh package)" && cargo publish -p "$$package" --all-features --locked --registry crates-io --allow-dirty --dry-run

publish:
	+@bash scripts/release/publish.sh "$(RELEASE_REMOTE)"

.PHONY: validation-tools-check
validation-tools-check: tools-check format-tools-check

# Ordered recursive calls preserve admission before dependent work under -j.
ci: shared-tooling-check
	+bash scripts/ci/run-validation-targets.sh --fail-fast $(VALIDATION_INPUTS) \
		validation-tools-check check-release-groups clippy check-timer-provider check-timer-dependencies check-timer-probes check-doc-links check-pins check-dependency-boundaries check-dependency-graphs check-release-commands \
		test-release-tooling test-formatting-evidence test-consumer-tooling test-ci-change-selection test-recovery-evidence test-release-status test-timers test-timer-dependencies \
		formatting-hook-check fmt-check check test-jobs test-consumers \
		check-consumer check-msrv check-wasm docs-check package-all

.PHONY: release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
version:
	+@if test -n "$${RELEASE_GROUP:-}"; then bash scripts/release/group.sh version; else \
		for group in jobs timers; do printf '%s: ' "$$group"; RELEASE_GROUP="$$group" bash scripts/release/group.sh version || exit; done; fi

.PHONY: release-status
export STATUS_OFFLINE ?= 0
export STATUS_JSON ?= 0
release-status:
	+@perl scripts/release/status.pl

release-version:
	@bash scripts/release/metadata.sh version

release-preflight:
	+@bash scripts/release/metadata.sh preflight

release-verify:
	+CARGO_NET_OFFLINE=true VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" bash scripts/ci/run-validation-targets.sh --fail-fast $(VALIDATION_INPUTS) ci install-testkit-tools install-consumer-server test-timer-watchdog test-timer-cohorts
	+@if test -n "$${RELEASE_SOURCE:-}"; then bash scripts/release/metadata.sh validated; fi

release-prepare-version:
	+@bash scripts/release/metadata.sh prepare

release-prepared-check release-committed-check release-tagged-check release-push-check:
	+@bash scripts/release/metadata.sh check

release-commit-check:
	+@bash scripts/release/metadata.sh commit-check

release-files:
	@bash scripts/release/group.sh files

# Timers product checks share this workspace's lock, compilers and Testkit setup.
.PHONY: test-timers test-timer-tooling check-timer-provider check-timer-dependencies check-timer-probes test-timer-watchdog test-timer-cohorts
check-timer-provider:
	bash scripts/ci/check-timer-provider.sh
check-timer-dependencies:
	+bash scripts/ci/check-timer-dependencies.sh
.PHONY: test-timer-dependencies
test-timer-dependencies:
	+bash scripts/ci/test-timer-dependencies.sh
test-timers:
	+$(MINIMUM_RUST) cargo test -p ic-timers --all-targets --locked --offline
	+$(MINIMUM_RUST) cargo test -p ic-timers --doc --locked --offline
	+$(MINIMUM_RUST) cargo test -p ic-timers-test-protocol -p ic-timers-runtime-probe --lib --locked --offline
	+$(MAKE) --no-print-directory test-timer-tooling
test-timer-tooling:
	+$(MINIMUM_RUST) cargo test -p ic-timers-pocketic --lib --bin ic-timers-pocketic --locked --offline
check-timer-probes:
	+$(MINIMUM_RUST) cargo clippy -p ic-timers-pocketic -p ic-timers-runtime-probe -p ic-timers-test-protocol --all-targets --locked --offline -- -D warnings
	+$(MINIMUM_RUST) cargo check -p ic-timers -p ic-timers-runtime-probe --target wasm32-unknown-unknown --locked --offline
	+@set -e; for selection in baseline once after-completion watchdog; do \
		$(MINIMUM_RUST) cargo clippy -p ic-timers-size-probe --no-default-features --features "$$selection" --all-targets --locked --offline -- -D warnings; \
	done
test-timer-watchdog: consumer-server-check
	+@cli="$$( $(TESTKIT_TOOL) --check )" && server="$$( "$$cli" check )" && \
		POCKET_IC_BIN="$$server" $(MINIMUM_RUST) cargo run -p ic-timers-pocketic --locked --offline -- watchdog
test-timer-cohorts: consumer-server-check
	+@cli="$$( $(TESTKIT_TOOL) --check )" && server="$$( "$$cli" check )" && \
		POCKET_IC_BIN="$$server" $(MINIMUM_RUST) cargo run -p ic-timers-pocketic --locked --offline -- cohorts
