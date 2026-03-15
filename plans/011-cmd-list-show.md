# Commands: list & show

Implement `tomb-cli/src/commands/list.rs` and `tomb-cli/src/commands/show.rs`.

Prerequisite: 003-config.md, 007-file-io.md, 008-rollover.md

## 1. `tomb list [--due <filter>] [--context <ctx>]`

- [ ] Resolve config → get all file paths
- [ ] For each file: read, rollover if managed, collect tasks
- [ ] Flatten tasks into a display list with metadata: `(task, file_path, section, context)`
- [ ] Apply filters:
  - `--due today` → tasks in Today section or date == today
  - `--due this-week` → tasks due within 7 days
  - `--due overdue` → tasks in past date sections (before today)
  - `--context <ctx>` → only tasks from files matching context
- [ ] Group output by section (Today, then dates, then Backlog)
- [ ] Print format per design doc:

```
  Today (2026-02-25)
  [/] Design the API                        t3k9
  [ ] Write the parser                      a1b2

  Backlog
  [ ] Write man page                        m2v7
  [ ] Something I jotted down               (no id)

  1 task without an ID. Run `tomb sync` to assign.
```

- [ ] Show `(no id)` for tasks without IDs
- [ ] Footer: count of tasks without IDs if any

## 2. `tomb show <id>`

- [ ] Resolve ID prefix across all files
- [ ] Read and parse the target file
- [ ] Find the task by ID (recursive search)
- [ ] Print full details per design doc:

```
  [/] Design the API ^t3k9
  Targeting OpenAPI 3.1 spec.

  Subtasks:
    [x] Define endpoints ^e2f1
    [ ] Write OpenAPI spec ^g3h4

  File:     ~/notes/tasks/work.md
  Section:  Today
  Context:  work
```

- [ ] Show description text
- [ ] Show subtasks with markers and IDs
- [ ] Show file path, section name, context

## 3. Wire into CLI dispatch

- [ ] Update `main.rs` match arms for `List` and `Show`

## 4. Tests

- [ ] `list` shows tasks from all configured files
- [ ] `list --due today` filters correctly
- [ ] `list --context work` filters by context
- [ ] `list` triggers rollover on managed files
- [ ] `list` shows `(no id)` for ID-less tasks
- [ ] `show` displays full task details with description and subtasks
- [ ] `show` with prefix resolution
- [ ] `show` error on unknown ID

## Verify

- [ ] `cargo test -p tomb-cli -- list` and `cargo test -p tomb-cli -- show`
- [ ] Manual: `tomb list`, `tomb list --due today`, `tomb show <id>`
