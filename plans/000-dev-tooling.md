# Dev Tooling & CI/CD

Set up development tooling, GitHub workflows, and dependency management. Based on patterns from the basalt project, adapted for tomb.

Prerequisite: none (can be done in parallel with 001-cargo-root)

## 1. Rust toolchain pinning

- [x] Create `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.94.0"
components = ["rust-std", "rust-analyzer", "rustfmt", "clippy"]
```

## 2. `.gitignore`

- [x] Create `.gitignore`

> Used `cargo init` generated `.gitignore` as the base instead of a
> minimal hand-written one. Adds `debug/` and keeps helpful comments.
> `.obsidian` excluded since not currently needed.

## 3. Cargo profiles

- [x] Add CI profile to workspace `Cargo.toml`:

```toml
[profile.ci]
inherits = "dev"
opt-level = 0

[profile.dev]
split-debuginfo = "unpacked"

[profile.dev.build-override]
opt-level = 3
```

## 4. Makefile

- [x] Create `Makefile` with common dev tasks:

```makefile
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
	git-cliff --prepend CHANGELOG.md
```

## 5. GitHub workflows

### 5a. `.github/workflows/test.yml` — CI on push/PR

- [x] Triggers: push to main, PRs (opened/synchronize/reopened), workflow_dispatch
- [x] Concurrency: cancel in-progress runs for same PR/ref
- [x] Steps: checkout, toolchain install, cargo cache, format check, cargo check, clippy, test, build + package
- [x] Use `--locked` and `--profile ci`
- [x] Pin all actions with SHA hashes (not tags)
- [x] Set `permissions: {}`  and `persist-credentials: false`

### 5b. `.github/workflows/build.yml` — cross-platform build (reusable)

- [x] `workflow_call` trigger (called by release)
- [x] Matrix: linux (x86_64 gnu/musl, aarch64 gnu/musl, armv7), macOS (x86_64, aarch64), Windows (gnu, msvc)
- [x] Build release binary for each target
- [x] Package as tar.gz (unix) or zip (windows) with sha256
- [x] Upload as artifacts

### 5c. `.github/workflows/release.yml` — GitHub releases

- [x] Triggers: push tag `v*.*.*`, workflow_dispatch with tag input
- [x] Calls build.yml, then downloads artifacts and uploads to GitHub release
- [x] Permissions: `contents: write`

> Tag pattern uses `tomb/v*.*.*` instead of `v*.*.*` to support a
> monorepo-style tag namespace.

### 5d. `.github/workflows/workflow-security.yml` — workflow linting

- [x] Triggers: push to main, PRs
- [x] Steps: actionlint, pinact (pin check), zizmor (security audit)
- [x] Sparse checkout `.github/workflows` only

## 6. Renovate (dependency updates)

### 6a. `renovate-config.json`

- [x] Extend `config:best-practices`, disable dashboard, daily schedule
- [x] `rebaseWhen: behind-base-branch`
- [x] `minimumReleaseAge: 3 days`, `osvVulnerabilityAlerts: true`
- [x] Disable semantic commits (we use our own style)
- [x] Lock file maintenance enabled
- [x] Custom managers:
  - `rust-toolchain.toml` channel version
  - Renovate self-version in workflow
  - zizmor version in workflow
- [x] Package rules:
  - Automerge minor/patch for renovate bot and GitHub Actions
  - Automerge patch for all GitHub Actions

### 6b. `.github/workflows/renovate.yml`

- [x] Self-hosted renovate via `renovatebot/github-action`
- [x] Daily schedule + push to main + manual dispatch
- [x] Sparse checkout (only config files + Cargo manifests)
- [x] Cache renovate repository cache between runs
- [x] Requires GitHub App token (app ID + private key in secrets)

### 6c. `.github/workflows/renovate-validate.yml`

- [x] Validates `renovate-config.json` on changes
- [x] Runs renovate-config-validator in Docker

### 6d. `.github/workflows/renovate-auto-approve.yml`

- [x] Auto-approves renovate PRs that have auto-merge enabled
- [x] Uses a separate GitHub App for approval (to satisfy branch protection requiring reviews)

## 7. CODEOWNERS

- [x] Create `.github/CODEOWNERS`:

```
* @erikjuhani
```

## 8. Changelog with git-cliff

- [x] Create `cliff.toml` configured for non-conventional commits
- [x] Commit parsers using `Changelog:` tags (added, changed, fixed, removed, etc.)
- [x] Filter out merge commits and renovate/dependabot commits into Dependencies group
- [x] Template linking commits and PRs to GitHub

## Verify

- [ ] `make check` passes locally
- [ ] Push to GitHub → test workflow runs green
- [ ] Renovate config validates
- [ ] `git-cliff` generates a changelog
