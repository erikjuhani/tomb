# Config Resolution

Implement `tomb-cli/src/config.rs` — locate and parse `.tomb.toml` / global config, resolve file paths, and provide file/inbox lookups to commands.

Prerequisite: 001-cargo-root.md (error types and model types exist)

## 1. Module scaffolding

- [ ] Create `tomb-cli/src/config.rs`
- [ ] Add `pub mod config;` to `lib.rs`
- [ ] Add `Config` error variant to `TombError` if needed
- [ ] Verify: `cargo check -p tomb-cli`

## 2. Config data model

- [ ] `FileEntry` struct: `path: Option<PathBuf>`, `glob: Option<String>`, `context: Option<String>`
- [ ] `InboxEntry` struct: `path: PathBuf`
- [ ] `Config` struct: `files: Vec<FileEntry>`, `inbox: Vec<InboxEntry>`, `root_dir: PathBuf` (directory containing the config file)
- [ ] Derive `serde::Deserialize` on all config structs

## 3. Config file discovery — `Config::resolve(start_dir)`

- [ ] Walk up from `start_dir` looking for `.tomb.toml`
- [ ] If found, parse it and set `root_dir` to its parent
- [ ] If not found, fall back to `~/.config/tomb/config.toml` (use `etcetera::choose_base_strategy()` / `config_dir()`)
- [ ] If neither exists, return a default empty config
- [ ] Return `TombError::Config` on parse failures

## 4. Path resolution

- [ ] `resolve_path(base: &Path, raw: &str) -> PathBuf` — expand `~` to home dir, resolve relative paths against `base`
- [ ] Apply to all `FileEntry.path` and `InboxEntry.path` after parsing
- [ ] Glob expansion: for entries with `glob`, use `glob` crate to expand patterns at runtime (add `glob` to dependencies)

## 5. File listing — `Config::resolve_files()`

- [ ] Expand all explicit paths and glob patterns into a deduplicated `Vec<ResolvedFile>`
- [ ] `ResolvedFile` struct: `path: PathBuf`, `context: Option<String>`
- [ ] Dedup by canonical path — explicit entries win over glob matches
- [ ] Context resolution order: explicit config > frontmatter (deferred to caller) > parent dir name

## 6. Inbox resolution — `Config::nearest_inbox(start_dir)`

- [ ] Return the inbox file closest to `start_dir` (per-repo inbox if inside a repo, otherwise first global inbox)
- [ ] Return `None` if no inboxes configured

## 7. Tests

- [ ] Parse a minimal `.tomb.toml` with `[[files]]` and `[[inbox]]`
- [ ] Walk-up discovery finds `.tomb.toml` in ancestor directory
- [ ] Falls back to global config when no `.tomb.toml` found
- [ ] `~` expansion works correctly
- [ ] Relative paths resolve against config file location
- [ ] Glob expansion picks up matching files
- [ ] Dedup: explicit entry wins over glob match for same file
- [ ] `nearest_inbox` returns per-repo inbox when inside repo

## Verify

- [ ] `cargo test -p tomb-cli -- config` — all config tests pass
- [ ] Manual: create a `.tomb.toml`, run `tomb list` (should resolve config without errors, even if list itself is still a stub)
