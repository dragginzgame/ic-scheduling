.DEFAULT_GOAL := help
export RELEASE_DELIVERY ?= direct
export RELEASE_GROUP
override RELEASE_SETUP_TARGET := release-candidate-setup
override TOKO_TOOLING_ROOT := $(CURDIR)
override RELEASE_DELIVERY_POLICY := $(CURDIR)/ci/release-delivery.txt
# Optimized qualification uses Binaryen; no other IC executable is required.
IC_TOOLS := wasm-opt
LOCAL_TOOL_INSTALL_TARGETS := install-ic-tools
LOCAL_TOOL_CHECK_TARGETS := ic-tools-check
export PATH := $(CURDIR)/.tools/ic/bin:$(PATH)
FORMAT_RUST_ARGS := --extra-workspace qualification
include make/consumer.mk

toko-tooling-check: check-qualification-tooling
.PHONY: check-qualification-tooling cloc-qualification
check-qualification-tooling:
	+bash scripts/ci/check-qualification-tooling.sh
cloc-qualification:
	+bash scripts/dev/cloc.sh --manifest qualification/Cargo.toml
# Source cleanup may write before capture; admit tooling before that prerequisite too.
clean-empty-source-dirs: toko-tooling-check
# Refuse absent, empty or unknown selections before prerequisites or setup run.
ifneq ($(filter release-patch release-minor release-major release-resume publish publish-check package,$(MAKECMDGOALS)),)
ifneq ($(filter release-patch release-minor release-major,$(MAKECMDGOALS)),)
ifneq ($(RELEASE_BATCH_ACTIVE),1)
ifneq ($(strip $(RELEASE_GROUPS)),)
release_outer_frozen := 1
endif
endif
endif
ifeq ($(release_outer_frozen),1)
release_group_comma := ,
release_selected_groups := $(subst $(release_group_comma), ,$(RELEASE_GROUPS))
ifneq ($(words $(RELEASE_GROUPS)),1)
$(error Frozen release groups must be a comma-separated roster)
endif
ifneq ($(filter-out jobs timers,$(release_selected_groups)),)
$(error Select RELEASE_GROUP=jobs or timers, or RELEASE_GROUPS=timers,jobs)
endif
ifneq ($(words $(release_selected_groups)),$(words $(sort $(release_selected_groups))))
$(error Frozen release groups must be unique)
endif
ifneq ($(findstring $(release_group_comma)$(release_group_comma),$(RELEASE_GROUPS)),)
$(error Frozen release groups must be a comma-separated roster)
endif
ifneq ($(filter $(release_group_comma)% %$(release_group_comma),$(RELEASE_GROUPS)),)
$(error Frozen release groups must be a comma-separated roster)
endif
ifneq ($(strip $(PACKAGE)),)
ifneq ($(words $(release_selected_groups)),1)
$(error PACKAGE cannot override a multi-group frozen release roster)
endif
ifneq ($(PACKAGE),ic-$(firstword $(release_selected_groups)))
$(error PACKAGE must belong to the explicitly selected release group)
endif
endif
ifneq ($(strip $(RELEASE_PUBLISH_TARGET)),)
ifneq ($(RELEASE_PUBLISH_TARGET),publish)
$(error RELEASE_PUBLISH_TARGET must be empty or publish)
endif
endif
else
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
endif
TESTKIT_TOOL = bash scripts/dev/testkit-tool.sh
MINIMUM_RUST = bash scripts/dev/minimum-rust.sh
RELEASE_METADATA_LOCKFILE ?= $(CURDIR)/qualification/Cargo.lock
RELEASE_METADATA_TOOL = bash scripts/dev/install-rust-tools.sh --package ic-build --lockfile "$(RELEASE_METADATA_LOCKFILE)" --bin ic-build-release-metadata --profile debug --features cli
VALIDATION_INPUTS := --input Cargo.toml --input Cargo.lock --input qualification/Cargo.toml --input qualification/Cargo.lock --input ci/workspace-manifests.txt --input scripts/ci/check-qualification-tooling.sh --input scripts/ci/check-workspace-inheritance.sh --input rust-toolchain.toml \
	--input .toko-tooling.snapshot --input Makefile --input scripts/dev/testkit-tool.sh --input scripts/dev/minimum-rust.sh --input ci/tool-versions.env \
	--input ci/ic-tools.tsv --input ci/release-delivery.txt --input ci/dependency-pinning-exceptions.json --input ci/cargo-dependency-boundaries.json --input ci/candid-boundary.json --input ci/cbor-dependencies.json --input ci/cargo-release-groups.json \
	--input .github/workflows/ci.yml --input .github/workflows/recovery.yml --input ci/docs-only-paths.txt --input scripts/ci/classify-ci-change.pl \
	--input ci/release-validation.json --input scripts/release/validation.pl --input scripts/release/verify.sh --input scripts/release/ci.sh \
	--input scripts/release/candidate-setup.sh --input scripts/release/source-lock.sh \
	--input scripts/release/metadata.sh --input scripts/release/publish.sh --input scripts/release/publication-lint.sh \
	$(foreach manifest,$(shell cat ci/workspace-manifests.txt),--input $(manifest))

# One policy roster feeds normal CI and complete selected release plans.

.PHONY: release-validation-stages test-release-validation
release-validation-stages:
	@perl scripts/release/validation.pl release-stages

test-release-validation:
	+perl scripts/release/validation-fixture.pl

.PHONY: help version install-hooks check test-jobs test-consumers check-msrv check-wasm clippy docs-check check-doc-links toko-tooling-check check-pins check-release-commands test-release-tooling test-formatting-evidence package publish-check publish ci
help:
	@echo "IC Scheduling: select RELEASE_GROUP=jobs or timers; frozen batches also accept RELEASE_GROUPS=timers,jobs"
	@echo "Timers: test-timers, test-timer-tooling, check-timer-provider, check-timer-dependencies, check-timer-probes"
	@echo "Explicit Timers IC qualification: test-timer-watchdog, test-timer-cohorts"
	@echo "Focused selections: see README (Consumer qualification) for package/test/target commands"
	@echo "Aggregates: check, test-jobs, test-consumers, check-msrv, check-wasm, clippy, docs-check, fmt-check"
	@echo "Consumer: check-consumer (native, Wasm, harness compilation); build-consumer (Wasm artifact)"
	@echo "Explicit IC qualification: install-testkit-tools; install-consumer-server; test-canister[-optimized]"
	@echo "Metadata: toko-tooling-check, check-doc-links, check-pins, check-candid-boundary, check-dependency-boundaries, check-dependency-graphs, check-release-commands"
	@echo "Read-only readiness: release-status (STATUS_OFFLINE=1, STATUS_JSON=1; optional RELEASE_GROUP)"
	@echo "Tooling fixtures: version, test-release-tooling, test-formatting-evidence, test-consumer-tooling"
	@echo "Release fixtures: test-release-tooling-{retention,delivery,recovery,metadata,publication,frozen}; explicit live qualification: test-release-tooling-frozen-installed"
	@echo "Hook qualification: formatting-hook-check (disposable Git copies; prepared formatters required)"
	@echo "Registry preparation: package (offline), publish-check (registry dry run; no upload)"
	@echo "Explicit setup: install-tools, tools-check, install-hooks"
	@echo "Release editor: install-release-tools; release-tools-check (offline-admitted path)"
	@echo "Complete gate: ci (explicit request or configured CI)"
	@echo "Group recipe selections: clippy-{jobs,timers}, docs-{jobs,timers}, package-{jobs,timers}"
	@echo "Authorized release: release-patch, release-minor, release-major (requires a configured remote)"
	@echo "Frozen publication: add RELEASE_PUBLISH_TARGET=publish when starting the release batch"
	@echo "Authorized publication: publish (clean HEAD, matching pushed annotated tag; crates.io)"
	@echo "Frozen status/recovery: release-batch-{status,resume,reconcile,retire} BATCH=batch-XXXXXXXX"
	@echo "Legacy same-checkout recovery: RELEASE_GROUP=... release-resume VERSION=X.Y.Z"

check:
	+cargo check -p ic-jobs --all-targets --all-features --locked --offline

test-jobs:
	+cargo test -p ic-jobs --tests --all-features --locked --offline

test-consumers:
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --test composition --test stored_wire --locked --offline
	+$(MINIMUM_RUST) cargo run -p ic-jobs --example persisted_job --locked --offline

.PHONY: check-consumer build-consumer install-testkit-tools testkit-tools-check install-consumer-server consumer-server-check test-canister test-canister-optimized
check-consumer:
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --lib --test api_wire --locked --offline
	+$(MAKE) --no-print-directory build-consumer
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --bin jobs-capture-artifact --test recovery --features harness --locked --offline

build-consumer:
	+$(MINIMUM_RUST) cargo build --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline

.PHONY: build-recovery-tools
build-recovery-tools:
	+$(MINIMUM_RUST) cargo build --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --bin jobs-capture-artifact --locked --offline

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
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --test recovery --features harness --locked --offline --no-run
	+bash scripts/ci/test-canister.sh $(if $(filter test-canister-optimized,$@),--optimized)

.PHONY: rust-policy-check
rust-policy-check:
	+bash scripts/dev/minimum-rust.sh --check

check-msrv: rust-policy-check
	+$(MINIMUM_RUST) cargo check -p ic-jobs --lib --locked --offline
	+$(MINIMUM_RUST) cargo check --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --test composition --test stored_wire --locked --offline

check-wasm:
	+$(MINIMUM_RUST) cargo check -p ic-jobs --lib --target wasm32-unknown-unknown --locked --offline
	+$(MINIMUM_RUST) cargo check --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --lib --target wasm32-unknown-unknown --locked --offline

.PHONY: clippy-jobs clippy-timers docs-jobs docs-timers package-jobs package-timers
clippy:
	+$(MAKE) --no-print-directory clippy-jobs
	+$(MAKE) --no-print-directory clippy-timers

clippy-jobs: rust-policy-check
	+cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings
	+cargo clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer -p ic-jobs-canister-tests --all-targets --features ic-jobs-canister-tests/harness --locked --offline -- -D warnings
	+cargo clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --target wasm32-unknown-unknown --locked --offline -- -D warnings

clippy-timers: rust-policy-check
	+cargo clippy -p ic-timers --all-targets --locked --offline -- -D warnings

docs-check:
	+$(MAKE) --no-print-directory docs-jobs
	+$(MAKE) --no-print-directory docs-timers

docs-jobs:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-jobs --all-features --locked --offline --no-deps
	+cargo test -p ic-jobs --doc --all-features --locked --offline

docs-timers:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-timers --all-features --locked --offline --no-deps

check-doc-links:
	perl scripts/ci/check-documentation-links.pl --root . *.md releases/history.md releases/*/CHANGELOG.md docs/*.md docs/changelog/*.md docs/principles/*.md docs/status/*.md rules/*.md audits/*.md tasks/*.md $(shell rg --files docs/timers -g '*.md') crates/ic-timers/SAFETY.md crates/ic-timers/CHANGELOG.md $(wildcard $(patsubst %/Cargo.toml,%/README.md,$(shell cat ci/workspace-manifests.txt))) $(wildcard $(patsubst %/Cargo.toml,%/tests/fixtures/README.md,$(shell cat ci/workspace-manifests.txt)))

check-pins:
	+bash scripts/ci/check-dependency-pins.sh
	+bash scripts/ci/check-workspace-inheritance.sh

.PHONY: check-candid-boundary check-dependency-boundaries check-release-groups package-all
check-release-groups:
	+@set -e; metadata="$$(mktemp)"; trap 'rm -f "$$metadata"' EXIT; \
		yq -p toml -o json '.' Cargo.toml > "$$metadata"; \
		jq -e --slurpfile root "$$metadata" '($$root[0].workspace.package | has("version") | not) and all(.groups[]; .version == "package" and .changelog == ("releases/" + .name + "/CHANGELOG.md"))' ci/cargo-release-groups.json > /dev/null
	+bash scripts/ci/cargo-release-group.sh plan jobs > /dev/null
	+bash scripts/ci/cargo-release-group.sh plan timers > /dev/null

check-candid-boundary:
	+perl scripts/ci/check-candid-boundary.pl --cbor "$(CURDIR)" ci/candid-boundary.json crates qualification

# One common source inventory per plan, before direct-dependency admission.
check-dependency-boundaries: check-candid-boundary
	+bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)" --external-ic
	+bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)/qualification" --external-ic

# Jobs owns these concrete exclusions; Shared owns observation and matching.
HOST_GRAPH_FORBIDDEN_PATTERN := ^ic-(?:build(?:-|$$)|host-|testkit(?:-|$$))
CORE_GRAPH_FORBIDDEN := clap ctrlc candid candid_derive candid_parser ic-cdk ic-cdk-macros \
	ic-cdk-executor ic-cdk-timers ic-platform ic-platform-pocket-ic ic-platform-runtime ic-platform-types ic-platform-macros ic-platform-runtime-guard ic0 ic-memory ic-stable-structures ic-storage \
	ic-timers ic-metrics ic-logging ic-wire ic-wire-derive postcard \
	 ic-jobs-test-consumer ic-jobs-canister-tests pocket-ic serde_json serde_test
QUALIFICATION_GRAPH_FORBIDDEN := ic-cdk-executor ic0 ic-platform ic-platform-pocket-ic clap ctrlc serde_json ic-memory ic-stable-structures candid candid_derive candid_parser ic-cdk ic-agent \
	 ic-jobs-canister-tests pocket-ic tokio
# Jobs and Timers share one Runtime owner; Storage keeps its allocation facade.
# Keep one executor, raw-system provider and entrypoint generator, without
# registry Principal conversion or hashing in the deployed graph.
# DTOs, Runtime, Endpoint and Testkit share canonical Types 0.19.
.PHONY: check-dependency-graphs
check-dependency-graphs:
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid-pattern '$(HOST_GRAPH_FORBIDDEN_PATTERN)' \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)" --package ic-jobs \
		--target wasm32-unknown-unknown --all-features \
		--forbid-pattern '$(HOST_GRAPH_FORBIDDEN_PATTERN)' \
		$(foreach dependency,$(CORE_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)/qualification" --package ic-jobs-test-consumer \
		--target wasm32-unknown-unknown --all-features \
		--require ic-timers=1 --require ic-platform-runtime-guard=1 \
		--require ic-platform-runtime=1 --require ic-platform-types=1 --require ic-platform-macros=1 \
		--forbid sha2 --forbid-feature ic-platform/principal-external \
		--forbid-pattern '$(HOST_GRAPH_FORBIDDEN_PATTERN)' \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)/qualification" --package ic-jobs-test-consumer \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --all-features \
		--forbid ic-cdk --forbid ic-platform --forbid ic-platform-runtime --forbid ic-platform-macros --forbid ic-platform-runtime-guard --forbid-feature ic-platform/cdk \
		--forbid ic-jobs --forbid ic-memory --forbid ic-storage --forbid ic-timers --forbid serde_json \
		--forbid ic-stable-structures --forbid ic-cdk-executor --forbid ic0 --forbid ic-metrics --forbid ic-logging \
		--forbid ic_principal --forbid sha2 \
		--forbid-pattern '$(HOST_GRAPH_FORBIDDEN_PATTERN)' \
		$(foreach dependency,$(QUALIFICATION_GRAPH_FORBIDDEN),--forbid $(dependency))
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)/qualification" --package ic-jobs-canister-tests \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" \
		--forbid candid --forbid candid_derive --forbid candid_parser \
		--forbid pocket-ic --forbid ic-platform-pocket-ic --forbid ic-testkit --forbid ic-jobs-test-consumer --forbid ic-wire --forbid ctrlc --forbid tokio --forbid-feature ic-platform/candid \
		--forbid-feature ic-platform/pocket-ic
	+perl scripts/ci/check-cargo-graph.pl --consumer "$(CURDIR)/qualification" --package ic-jobs-canister-tests \
		--target "$$(rustc -vV | sed -n 's/^host: //p')" --features harness --edges normal \
		--require ic-testkit=1 --require ic-platform-types=1 --forbid ic-platform-pocket-ic --forbid pocket-ic \
		--require ic-platform=1 --require ic-wire=1 --forbid ic-management-canister-types

check-release-commands:
	RELEASE_BATCH_ACTIVE=1 RELEASE_GROUP=jobs bash scripts/ci/check-release-commands.sh . make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh scripts/ci/check-release-delivery.sh ci/release-delivery.txt scripts/ci/check-qualification-tooling.sh Cargo.toml Cargo.lock qualification/Cargo.lock $(shell rg --files crates qualification -g '*.rs' -g Cargo.toml)

test-release-tooling: format-tools-check
	+@editor="$$( $(RELEASE_METADATA_TOOL) --check )" && \
		CARGO_RELEASE_METADATA_EDITOR="$$editor" CARGO_RELEASE_METADATA_MAX_BYTES=1048576 \
		bash scripts/release/test-tooling.sh

RELEASE_TOOLING_GROUPS := delivery recovery metadata publication frozen
.PHONY: test-release-tooling-retention $(addprefix test-release-tooling-,$(RELEASE_TOOLING_GROUPS))
test-release-tooling-retention:
	+bash scripts/release/test-tooling.sh retention

$(addprefix test-release-tooling-,$(filter-out frozen,$(RELEASE_TOOLING_GROUPS))): format-tools-check
	+@editor="$$( $(RELEASE_METADATA_TOOL) --check )" && \
		CARGO_RELEASE_METADATA_EDITOR="$$editor" CARGO_RELEASE_METADATA_MAX_BYTES=1048576 \
		bash scripts/release/test-tooling.sh $(@:test-release-tooling-%=%)

test-release-tooling-frozen: host-tools-check
	+bash scripts/release/test-tooling.sh frozen

.PHONY: test-release-tooling-frozen-installed
test-release-tooling-frozen-installed: host-tools-check
	+FROZEN_TEST_INSTALLED=1 bash scripts/release/test-tooling.sh frozen

.PHONY: install-release-tools release-tools-check
.PHONY: release-candidate-setup
release-candidate-setup:
	+bash scripts/release/candidate-setup.sh

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
	+$(MAKE) --no-print-directory package-jobs
	+$(MAKE) --no-print-directory package-timers

package-jobs:
	+cargo package -p ic-jobs --all-features --locked --offline --allow-dirty

package-timers:
	+cargo package -p ic-timers --all-features --locked --offline --allow-dirty

package:
	+@package="$$(bash scripts/release/group.sh package)" && cargo package -p "$$package" --all-features --locked --offline --allow-dirty

publish-check: toko-tooling-check
	+bash scripts/release/publication-lint.sh
	+@package="$$(bash scripts/release/group.sh package)" && cargo publish -p "$$package" --all-features --locked --registry crates-io --allow-dirty --dry-run

publish:
	+@bash scripts/release/publish.sh "$(RELEASE_REMOTE)"

.PHONY: validation-tools-check
validation-tools-check: tools-check format-tools-check

# Ordered recursive calls preserve admission before dependent work under -j.
ci:
	+bash scripts/release/ci.sh $(VALIDATION_INPUTS)

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
	+bash scripts/release/verify.sh

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
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-timers-test-protocol -p ic-timers-runtime-probe --lib --locked --offline
	+$(MAKE) --no-print-directory test-timer-tooling
test-timer-tooling:
	+$(MINIMUM_RUST) cargo test --manifest-path qualification/Cargo.toml -p ic-timers-pocketic --lib --bin ic-timers-pocketic --locked --offline
check-timer-probes:
	+$(MINIMUM_RUST) cargo clippy --manifest-path qualification/Cargo.toml -p ic-timers-pocketic -p ic-timers-runtime-probe -p ic-timers-test-protocol --all-targets --locked --offline -- -D warnings
	+$(MINIMUM_RUST) cargo check --manifest-path qualification/Cargo.toml -p ic-timers -p ic-timers-runtime-probe --target wasm32-unknown-unknown --locked --offline
	+@set -e; for selection in baseline once after-completion watchdog; do \
		$(MINIMUM_RUST) cargo clippy --manifest-path qualification/Cargo.toml -p ic-timers-size-probe --no-default-features --features "$$selection" --all-targets --locked --offline -- -D warnings; \
	done
test-timer-watchdog: consumer-server-check
	+@cli="$$( $(TESTKIT_TOOL) --check )" && server="$$( "$$cli" check )" && \
		POCKET_IC_BIN="$$server" $(MINIMUM_RUST) cargo run --manifest-path qualification/Cargo.toml -p ic-timers-pocketic --locked --offline -- watchdog
test-timer-cohorts: consumer-server-check
	+@cli="$$( $(TESTKIT_TOOL) --check )" && server="$$( "$$cli" check )" && \
		POCKET_IC_BIN="$$server" $(MINIMUM_RUST) cargo run --manifest-path qualification/Cargo.toml -p ic-timers-pocketic --locked --offline -- cohorts
