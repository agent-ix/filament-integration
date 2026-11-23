# =============================================================================
# Quire Integration Makefile
# =============================================================================

CARGO ?= cargo

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make figures          - Regenerate docs/images from docs/images/figures.json (ixd build)"
	@echo "  make figures-check    - Fail when docs/images are stale; writes nothing"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make kani             - Run real Kani proof and mutation integration tests"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo deny check licenses"
	@echo "  make audit-unsafe     - Enforce unsafe-block documentation through Clippy"
	@echo "  make ci               - All CI gates locally (fmt-check + lint + test + kani + deny + audit-unsafe)"

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: test
test:
	$(CARGO) test

.PHONY: kani
kani:
	$(CARGO) test --locked --test ir914_bound_export -- --ignored --test-threads=1

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check licenses

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

.PHONY: audit-unsafe
audit-unsafe: lint

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check lint test kani deny audit-unsafe

# =============================================================================
# Documentation figures
# =============================================================================
# docs/images/figures.json defines the figures; `ixd build` (agent-ix/ix-diagrams) renders them.
# Install it with: cargo install --git https://github.com/agent-ix/ix-diagrams ix-diagrams

IXD ?= ixd

.PHONY: figures
figures:
	$(IXD) build docs/images/figures.json --out docs/images

.PHONY: figures-check
figures-check:
	$(IXD) build docs/images/figures.json --out docs/images --check
