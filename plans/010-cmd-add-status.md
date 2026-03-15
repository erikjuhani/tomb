# Commands: add, done, start, cancel

Implement `tomb-cli/src/commands/add.rs` and `tomb-cli/src/commands/status.rs`.

Prerequisite: 003-config.md, 007-file-io.md, 006-id-system.md

## 1. `tomb add <title> [--due <date>] [--context <ctx>]`

- [ ] Resolve target file:
  - If `--context` given, find the managed file matching that context
  - If no context, use the first managed file from config (or error if none)
- [ ] Parse `--due` value:
  - `today` → `SectionKind::Today`
  - `tomorrow` → `SectionKind::Date(today + 1)`
  - `YYYY-MM-DD` → `SectionKind::Date(parsed)`
  - No `--due` → `SectionKind::Backlog`
- [ ] Create `Task` with `marker: Todo`, title, generated ID, no description, no children
- [ ] Use `mutate_managed_file` to:
  - Find or create the target section
  - Append task to the section
- [ ] Print confirmation: `Added: {title} ^{id}`

## 2. `tomb done <id>`

- [ ] Resolve ID prefix across all files (`resolve_id_across_files`)
- [ ] Use `mutate_managed_file` on the target file:
  - Find task by ID
  - Set marker to `Done`
  - Cascade: mark uncompleted children as `Cancelled`
- [ ] Print: `Done: {title} ^{id}`

## 3. `tomb start <id>`

- [ ] Same flow as `done` but set marker to `InProgress`
- [ ] No cascade needed
- [ ] Print: `Started: {title} ^{id}`

## 4. `tomb cancel <id>`

- [ ] Same flow as `done` but set marker to `Cancelled`
- [ ] Cascade: mark uncompleted children as `Cancelled`
- [ ] Print: `Cancelled: {title} ^{id}`

## 5. Shared helpers

- [ ] `find_task_mut(file: &mut ManagedFile, id: &str) -> Option<&mut Task>` — recursive search through sections and children
- [ ] `cascade_cancel(task: &mut Task)` — mark uncompleted children as `Cancelled`, recurse

## 6. Wire into CLI dispatch

- [ ] Update `main.rs` match arms for `Add`, `Done`, `Start`, `Cancel`

## 7. Tests

- [ ] `add` with no flags → appends to Backlog
- [ ] `add --due today` → appends to Today section
- [ ] `add --due 2026-03-15` → creates/appends to date section
- [ ] `add --context work` → targets correct file
- [ ] `done` sets marker and cascades to children
- [ ] `start` sets marker, no cascade
- [ ] `cancel` sets marker and cascades
- [ ] ID prefix resolution works across files
- [ ] Error on ambiguous prefix
- [ ] Error on unknown ID

## Verify

- [ ] `cargo test -p tomb-cli -- add` and `cargo test -p tomb-cli -- status`
- [ ] Manual: `tomb add "Test task" --due today` → `tomb list` → `tomb done <id>` → `tomb list`
