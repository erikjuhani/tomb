.PHONY: fmt fmt-check check changelog

check:
	@$(MAKE) fmt-check
	cargo check --locked --profile ci --workspace --all-targets
	cargo clippy --profile ci --workspace --all-targets -- -D warnings
	cargo test --profile ci --workspace --all-targets
	cargo build --profile ci --workspace --all-targets

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

changelog:
	@if [ -z "$(crate)" ]; then echo "Error: crate parameter is required (e.g., CRATE=tomb-core)"; exit 1; fi
	@if [ -z "$(version)" ]; then echo "Error: version parameter is required (e.g., VERSION=0.1.0)"; exit 1; fi
	git-cliff -u --include-path "$(crate)/**" --tag "$(crate)/$(version)" --count-tags "$(crate)/v*" --prepend $(crate)/CHANGELOG.md
