# Tomb CLI MVP — Implementation Plan

## Context

Tomb is a markdown-based task manager with Obsidian compatibility. The v1 is a shell script (`tomb.sh`); this plan covers the Rust rewrite on the `v2` branch. The MVP targets the core workflow: managed markdown files with rollover, task CRUD, inbox capture, and interactive review.

## Tech Stack

- **Language:** Rust (edition 2021)
- **CLI:** clap 4 (derive macros)
- **TUI:** ratatui + crossterm (interactive review)
- **Config:** serde + toml
- **Dates:** chrono
- **Locking:** fs2 (advisory flock)
- **IDs:** rand (4-char alphanumeric)

## MVP Scope

**In scope:** `init`, `add`, `done`, `start`, `cancel`, `list`, `show`, `sync`, `inbox`, `review` — managed mode only.

**Out of scope:** tracked mode, plugins, LLM-assisted review, globs, search, log, push/pull, mv, edit, `--json`, config CLI.

## Module Structure

```
Cargo.toml              — workspace root
crates/
  pulldown-cmark-task-marker/              — forked pulldown-cmark 0.13.1 (sub-crate)
    Cargo.toml
    src/
      scanners.rs       — our change: TaskListMarker(char) instead of bool
      parse.rs          — plumb char through ItemBody
      lib.rs            — plumb char through Event enum
      html.rs           — update HTML rendering for char
      firstpass.rs      — plumb char through
      ...               — rest unchanged from upstream
src/
  main.rs               — entry point, clap dispatch
  lib.rs                — pub mod declarations
  cli.rs                — clap derive structs
  error.rs              — TombError enum, Result alias
  config.rs             — .tomb.toml resolution (walk-up + global fallback)
  model.rs              — Frontmatter, TaskMarker, Task, Section, ManagedFile
  parser.rs             — pulldown-cmark events → ManagedFile
  renderer.rs           — ManagedFile → markdown string
  rollover.rs           — date-based rollover logic
  id.rs                 — ID generation, prefix matching, duplicate detection
  io.rs                 — atomic writes (tmp+rename), advisory file locks, optimistic concurrency
  commands/
    mod.rs              — re-exports
    init.rs             — tomb init
    add.rs              — tomb add
    status.rs           — tomb done / start / cancel
    list.rs             — tomb list
    show.rs             — tomb show
    sync.rs             — tomb sync
    inbox.rs            — tomb inbox
    review.rs           — tomb review (ratatui TUI)
```

## Parser Strategy: Forked pulldown-cmark

We fork pulldown-cmark 0.13.1 as a workspace sub-crate (`crates/pulldown-cmark-task-marker/`). This gives us a proper CommonMark parser that handles headings, list items, code blocks, links, frontmatter, and all edge cases correctly. We extend it with one small change: custom task list markers `[/]` and `[-]`.

### What we change in the fork

Two changes, fully non-breaking:

1. **New `Event::ExtendedTaskListMarker(char)` variant** — added alongside the existing `TaskListMarker(bool)` which remains untouched. When extended mode is active, `ExtendedTaskListMarker(char)` is emitted instead of `TaskListMarker(bool)`.

2. **New `Options::ENABLE_EXTENDED_TASK_MARKERS` flag** — controls which variant the parser emits:

| Option state | Scanner accepts | Emits |
|---|---|---|
| `ENABLE_TASKLISTS` only (default) | `x`, `X`, space | `TaskListMarker(bool)` — unchanged |
| `ENABLE_TASKLISTS + ENABLE_EXTENDED_TASK_MARKERS` | Any char except `]` | `ExtendedTaskListMarker(char)` |

Standard GFM behavior is fully preserved. Existing code matching on `TaskListMarker(bool)` compiles and works identically.

**Files changed (~20 lines across 5 files):**
- `src/lib.rs` — `ENABLE_EXTENDED_TASK_MARKERS` option bit, `ExtendedTaskListMarker(char)` variant
- `src/scanners.rs` — new `scan_extended_task_list_marker` method returning `Option<char>`
- `src/parse.rs` — `ExtendedTaskListMarker(char)` in `ItemBody`, option check to pick scanner
- `src/firstpass.rs` — handle new variant in match arms
- `src/html.rs` — render new variant (`' '` → unchecked, else → checked)

### What pulldown-cmark gives us for free

- **Frontmatter**: `Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` → `Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle))` with the YAML text
- **Headings**: `Event::Start(Tag::Heading { level: H2, .. })` with text events
- **List items**: `Event::Start(Tag::Item)` properly nested
- **Task markers**: `Event::ExtendedTaskListMarker(char)` with `ENABLE_EXTENDED_TASK_MARKERS` option — `' '`, `'x'`, `'/'`, `'-'`, or any single char
- **Code blocks**: Properly parsed, no manual fence tracking needed
- **Links**: `Event::Start(Tag::Link { dest_url, title, .. })` with inline text
- **Source offsets**: `into_offset_iter()` gives `(Event, Range<usize>)` byte ranges for every event

### How we use it in tomb

Our `parser.rs` consumes pulldown-cmark events to build `ManagedFile`. The event stream gives us structured access to headings, list items, task markers, links, and code blocks — no regex needed for standard markdown constructs. We only need custom extraction for `^block-id` (Obsidian-specific, appears as trailing `Text` content).

For **rendering**, we still write our own `render_managed_file` since we need to output our specific format (not generic CommonMark). But the parser side is fully handled by pulldown-cmark.

For **round-tripping**: simple mutations (status change) use surgical edits via source offsets. Structural mutations (rollover, moves) do a full re-render from the model.

### Fork strategy: Workspace + Git Subtree

We use a **Cargo workspace** with a **git subtree** for the pulldown-cmark fork:

- **Workspace**: Root `Cargo.toml` declares `[workspace] members = ["crates/pulldown-cmark-task-marker"]`. The `tomb` binary crate depends on `pulldown-cmark-task-marker` via `path = "crates/pulldown-cmark-task-marker"`.
- **Git subtree**: The upstream pulldown-cmark source is added with `git subtree add --prefix=crates/pulldown-cmark-task-marker`. This embeds the full source in our repo while preserving a path to pull upstream changes.
- **Upstream sync**: `git subtree pull --prefix=crates/pulldown-cmark-task-marker <remote> <branch> --squash` merges upstream updates. Our changes (~10 lines across 4 files) are isolated to the task list marker type, so conflicts are unlikely.

Dependencies kept: `bitflags`, `memchr`, `unicase`. Stripped: `getopts`, `pulldown-cmark-escape`, examples, binaries — we only need the library.

## Implementation Phases

Each phase has a detailed plan in `plans/`:

| Plan | Phase |
|------|-------|
| [`000-dev-tooling`](plans/000-dev-tooling.md) | Dev tooling, CI/CD, Renovate, changelog |
| [`001-cargo-root`](plans/001-cargo-root.md) | Phase 1: Project skeleton |
| [`002-tomb-parser`](plans/002-tomb-parser.md) | Phase 1: pulldown-cmark fork |
| [`003-config`](plans/003-config.md) | Phase 2: Config resolution |
| [`004-parser`](plans/004-parser.md) | Phase 3: Markdown parser |
| [`005-renderer`](plans/005-renderer.md) | Phase 4: Markdown renderer |
| [`006-id-system`](plans/006-id-system.md) | Phase 5: ID system |
| [`007-file-io`](plans/007-file-io.md) | Phase 6: File I/O |
| [`008-rollover`](plans/008-rollover.md) | Phase 7: Rollover |
| [`009-cmd-init-sync`](plans/009-cmd-init-sync.md) | Phase 8: Commands — init, sync |
| [`010-cmd-add-status`](plans/010-cmd-add-status.md) | Phase 9: Commands — add, done, start, cancel |
| [`011-cmd-list-show`](plans/011-cmd-list-show.md) | Phase 10: Commands — list, show |
| [`012-cmd-inbox`](plans/012-cmd-inbox.md) | Phase 11: Command — inbox |
| [`013-cmd-review`](plans/013-cmd-review.md) | Phase 12: Command — review |
| [`014-integration-tests`](plans/014-integration-tests.md) | Phase 13: Integration tests |

### Phase 1: Project Skeleton + pulldown-cmark Fork

Detailed steps in [`001-cargo-root`](plans/001-cargo-root.md) and [`002-tomb-parser`](plans/002-tomb-parser.md).

1. Convert root `Cargo.toml` to workspace with all deps (cargo-root.md §1)
2. Create all tomb source files: error types, data model, CLI, command stubs (cargo-root.md §2)
3. Clean up v1 artifacts (cargo-root.md §3)
4. Add pulldown-cmark as git subtree into `crates/pulldown-cmark-task-marker/` (tomb-parser.md §1)
5. Strip fork to library-only, rename to pulldown-cmark-task-marker (tomb-parser.md §3)
6. Apply fork changes: `TaskListMarker(bool)` → `TaskListMarker(char)` (tomb-parser.md §4)

**Verify:** `cargo build` succeeds, `cargo run -- --help` shows subcommands, `cargo build -p pulldown-cmark-task-marker` compiles independently.

### Phase 2: Config Resolution

Detailed steps in [`003-config`](plans/003-config.md).

Implement `Config::resolve(start_dir)` — walk up from cwd for `.tomb.toml`, fall back to `~/.config/tomb/config.toml`. Parse with `toml::from_str`. Resolve file paths (expand `~`, relative paths). `nearest_inbox()` returns closest inbox file.

### Phase 3: Markdown Parser

Detailed steps in [`004-parser`](plans/004-parser.md).

Event-driven parser using our forked pulldown-cmark (`parse_managed_file`):
- Enable options: `ENABLE_TASKLISTS | ENABLE_YAML_STYLE_METADATA_BLOCKS`
- Use `into_offset_iter()` to get events with source byte ranges
- **Frontmatter**: `MetadataBlock(YamlStyle)` events → parse YAML key-value pairs for tomb fields
- **Headings**: `Heading { level: H2 }` events → classify as Today/Backlog/Date sections
- **Task list items**: `TaskListMarker(char)` events → map char to `TaskMarker` enum
- **Links**: `Tag::Link { dest_url, .. }` events → extract external links from task lines
- **Block IDs**: Extract `^xxxx` from trailing `Text` events on task lines (Obsidian-specific, not handled by pulldown-cmark)
- **Code blocks, nested lists, descriptions**: All handled correctly by pulldown-cmark's event stream — no manual fence tracking needed
- Build task trees from list item nesting (pulldown-cmark handles indentation)
- Store source byte ranges on tasks/sections for surgical edits

**Tests:** Round-trip the design doc examples. Test code blocks in descriptions. This is the most critical module.

### Phase 4: Markdown Renderer

Detailed steps in [`005-renderer`](plans/005-renderer.md).

`render_managed_file` — serialize `ManagedFile` back to markdown string. Frontmatter as YAML, sections with `##` headings, tasks with correct indentation, IDs, links, descriptions. Section sort function: Today → Backlog → future dates (desc) → past dates (desc).

**Tests:** Round-trip tests (parse → render → compare).

### Phase 5: ID System

Detailed steps in [`006-id-system`](plans/006-id-system.md).

- `generate_id(existing)` — random 4-char `[a-z0-9]`, retry on collision
- `resolve_prefix(prefix, all_ids)` — exact or unique prefix match, error if ambiguous
- `assign_missing_ids(file)` — walk all tasks, assign IDs where missing
- `find_duplicates(file)` — detect duplicate IDs
- `collect_ids(file)` — gather all IDs recursively

### Phase 6: File I/O

Detailed steps in [`007-file-io`](plans/007-file-io.md).

- `read_managed_file(path)` — shared lock, read, parse
- `write_managed_file(path, original, file)` — exclusive lock, conflict detection, tmp+rename
- `mutate_managed_file(path, closure)` — read → mutate → assign IDs → write with optimistic concurrency

### Phase 7: Rollover

Detailed steps in [`008-rollover`](plans/008-rollover.md).

`maybe_rollover(file, today)` — check `last_rollover` < today, then:
1. Old Today → date heading with `last_rollover` date
2. Completed tasks stay, done parents cascade `[-]` to uncompleted children
3. Uncompleted tasks from old Today → carry forward to new Today
4. Tasks due today (from date heading) → top of new Today
5. Uncompleted tasks from older date headings → Backlog
6. Remove empty date sections
7. Update `last_rollover` in frontmatter

**Tests:** Full rollover scenario from design doc (Feb 25 → Feb 26).

### Phase 8: Commands — `init`, `sync`

Detailed steps in [`009-cmd-init-sync`](plans/009-cmd-init-sync.md).

- `init` — create managed file with frontmatter + empty Today/Backlog, add to `.tomb.toml`
- `sync` — for each file: rollover if needed, assign missing IDs, fix duplicates

### Phase 9: Commands — `add`, `done`, `start`, `cancel`

Detailed steps in [`010-cmd-add-status`](plans/010-cmd-add-status.md).

- `add` — resolve target file by context, parse `--due` (today/tomorrow/date), append task to section
- `done`/`start`/`cancel` — resolve ID prefix across all files, set marker, cascade on done/cancel

### Phase 10: Commands — `list`, `show`

Detailed steps in [`011-cmd-list-show`](plans/011-cmd-list-show.md).

- `list` — read all files, rollover each, collect tasks, filter by `--due` and `--context`, print grouped
- `show` — resolve ID, print full task details with description, subtasks, file, section, context

### Phase 11: Command — `inbox`

Detailed steps in [`012-cmd-inbox`](plans/012-cmd-inbox.md).

- With text arg: append `- [ ] {text}` to nearest inbox file
- Without args: open inbox in `$EDITOR`

### Phase 12: Command — `review` (ratatui TUI)

Detailed steps in [`013-cmd-review`](plans/013-cmd-review.md).

Parse inbox into items (task lines and text paragraphs). Ratatui alternate screen with:
- Current item display, progress counter
- Action keys: `[a]dd` → context select → due date input → add to managed file, `[s]kip`, `[d]elete`, `[q]uit`
- On exit: rewrite inbox with only skipped/unprocessed items, print summary

### Phase 13: Integration Tests

Detailed steps in [`014-integration-tests`](plans/014-integration-tests.md).

End-to-end tests using `tempfile::TempDir`: init → add → list round-trip, rollover behavior, done cascade, sync ID assignment, inbox capture, prefix matching, conflict detection.

## Dependencies (Cargo.toml)

```toml
[workspace]
members = ["crates/pulldown-cmark-task-marker"]

[package]
name = "tomb"
version = "0.1.0"
edition = "2021"

[dependencies]
pulldown-cmark-task-marker = { path = "crates/pulldown-cmark-task-marker", default-features = false }
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
rand = "0.9"
ratatui = "0.29"
crossterm = "0.28"
thiserror = "2"
dirs = "6"
fs2 = "0.4"

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
```

## Verification

1. `cargo build` — compiles cleanly
2. `cargo test` — all unit and integration tests pass
3. Manual workflow test:
   ```bash
   tomb init tasks.md --context work
   tomb add "First task" --due today
   tomb add "Second task"
   tomb list
   tomb start <id>
   tomb done <id>
   tomb inbox "Quick thought"
   tomb review
   tomb sync
   ```
