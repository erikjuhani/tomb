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

- [ ] Add CI profile to workspace `Cargo.toml`:

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

- [ ] Create `Makefile` with common dev tasks:

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

- [ ] Triggers: push to main, PRs (opened/synchronize/reopened), workflow_dispatch
- [ ] Concurrency: cancel in-progress runs for same PR/ref
- [ ] Steps: checkout, toolchain install, cargo cache, format check, cargo check, clippy, test, build + package
- [ ] Use `--locked` and `--profile ci`
- [ ] Pin all actions with SHA hashes (not tags)
- [ ] Set `permissions: {}`  and `persist-credentials: false`

### 5b. `.github/workflows/build.yml` — cross-platform build (reusable)

- [ ] `workflow_call` trigger (called by release)
- [ ] Matrix: linux (x86_64 gnu/musl, aarch64 gnu/musl, armv7), macOS (x86_64, aarch64), Windows (gnu, msvc)
- [ ] Build release binary for each target
- [ ] Package as tar.gz (unix) or zip (windows) with sha256
- [ ] Upload as artifacts

### 5c. `.github/workflows/release.yml` — GitHub releases

- [ ] Triggers: push tag `v*.*.*`, workflow_dispatch with tag input
- [ ] Calls build.yml, then downloads artifacts and uploads to GitHub release
- [ ] Permissions: `contents: write`

### 5d. `.github/workflows/workflow-security.yml` — workflow linting

- [ ] Triggers: push to main, PRs
- [ ] Steps: actionlint, pinact (pin check), zizmor (security audit)
- [ ] Sparse checkout `.github/workflows` only

## 6. Renovate (dependency updates)

### 6a. `renovate-config.json`

- [ ] Extend `config:best-practices`, disable dashboard, daily schedule
- [ ] `rebaseWhen: behind-base-branch`
- [ ] `minimumReleaseAge: 3 days`, `osvVulnerabilityAlerts: true`
- [ ] Disable semantic commits (we use our own style)
- [ ] Lock file maintenance enabled
- [ ] Custom managers:
  - `rust-toolchain.toml` channel version
  - Renovate self-version in workflow
  - zizmor version in workflow
- [ ] Package rules:
  - Automerge minor/patch for renovate bot and GitHub Actions
  - Automerge patch for all GitHub Actions

### 6b. `.github/workflows/renovate.yml`

- [ ] Self-hosted renovate via `renovatebot/github-action`
- [ ] Daily schedule + push to main + manual dispatch
- [ ] Sparse checkout (only config files + Cargo manifests)
- [ ] Cache renovate repository cache between runs
- [ ] Requires GitHub App token (app ID + private key in secrets)

### 6c. `.github/workflows/renovate-validate.yml`

- [ ] Validates `renovate-config.json` on changes
- [ ] Runs renovate-config-validator in Docker

### 6d. `.github/workflows/renovate-auto-approve.yml`

- [ ] Auto-approves renovate PRs that have auto-merge enabled
- [ ] Uses a separate GitHub App for approval (to satisfy branch protection requiring reviews)

## 7. CODEOWNERS

- [ ] Create `.github/CODEOWNERS`:

```
* @erikjuhani
```

## 8. Changelog with git-cliff

- [ ] Create `cliff.toml` configured for non-conventional commits
- [ ] Commit parsers using `Changelog:` tags (added, changed, fixed, removed, etc.)
- [ ] Filter out merge commits and renovate/dependabot commits into Dependencies group
- [ ] Template linking commits and PRs to GitHub

## Verify

- [ ] `make check` passes locally
- [ ] Push to GitHub → test workflow runs green
- [ ] Renovate config validates
- [ ] `git-cliff` generates a changelog
