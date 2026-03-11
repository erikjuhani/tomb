# Commands: init & sync

Implement `src/commands/init.rs` and `src/commands/sync.rs`.

Prerequisite: 003-config.md, 007-file-io.md, 006-id-system.md, 008-rollover.md

## 1. `tomb init <path> [--context <ctx>]`

- [ ] Validate path: must not already exist (or prompt to overwrite)
- [ ] Create parent directories if needed
- [ ] Build a `ManagedFile` with:
  - Frontmatter: `tomb_mode: managed`, `tomb_version: 1`, `context` (if provided), `last_rollover: today`
  - Two empty sections: `Today`, `Backlog`
- [ ] Render to markdown string
- [ ] Write file (atomic write, no conflict check needed for new file)
- [ ] Add the file to the nearest `.tomb.toml` (or global config if no local config)
  - Read existing config, append `[[files]]` entry, write back
- [ ] Print confirmation: `Created {path}`

## 2. `tomb sync`

- [ ] Resolve config → get all file paths
- [ ] For each file:
  - [ ] Read and parse
  - [ ] Run `maybe_rollover(file, today)` if managed
  - [ ] Run `assign_missing_ids(file)`
  - [ ] Run duplicate detection → repair duplicates
  - [ ] Write back if any changes were made
- [ ] Print summary: `Synced {n} files. {ids_assigned} IDs assigned. {dupes_fixed} duplicates fixed. {rollovers} files rolled over.`

## 3. Wire into CLI dispatch

- [ ] Update `main.rs` match arms for `Commands::Init` and `Commands::Sync`
- [ ] Pass resolved config to sync

## 4. Tests

- [ ] `init` creates file with correct frontmatter and sections
- [ ] `init` with `--context` sets context in frontmatter
- [ ] `init` adds entry to `.tomb.toml`
- [ ] `sync` assigns IDs to tasks without them
- [ ] `sync` fixes duplicate IDs
- [ ] `sync` triggers rollover on managed files when date has changed
- [ ] `sync` is idempotent (running twice produces no changes)

## Verify

- [ ] `cargo test -p tomb -- init` and `cargo test -p tomb -- sync`
- [ ] Manual: `tomb init test.md --context test` → verify file contents → `tomb sync` → verify IDs assigned
