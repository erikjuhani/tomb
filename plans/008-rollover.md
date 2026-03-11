# Rollover

Implement `src/rollover.rs` — date-based task redistribution for managed files.

Prerequisite: 004-parser.md, 005-renderer.md (model types and round-trip capability)

## 1. Entry point — `maybe_rollover(file, today)`

- [ ] Check if `file.frontmatter.last_rollover < today`
- [ ] If no rollover needed, return `false`
- [ ] If `last_rollover` is absent, infer from file structure and log warning
- [ ] Perform rollover, update `last_rollover` to `today`, return `true`

## 2. Rollover logic

Implement the 9 rules from the design doc:

- [ ] **Rule 1**: Rename current `Today` section to a date heading using `last_rollover` date
- [ ] **Rule 2**: Create a new empty `Today` section at the top
- [ ] **Rule 3**: Tasks from date headings matching `today` → move to new `Today` (due today, inserted first)
- [ ] **Rule 4**: Uncompleted tasks from old Today → carry forward to new `Today` (after due-today tasks)
- [ ] **Rule 5**: Uncompleted tasks from date headings older than `last_rollover` → move to `Backlog`
- [ ] **Rule 6**: Completed task trees stay under their date heading
- [ ] **Rule 7**: Done parent with uncompleted children → mark children `[-]` (cancelled)
- [ ] **Rule 8**: Remove empty date sections (all tasks moved out)
- [ ] **Rule 9**: Preserve user ordering within Today on subsequent reads

## 3. Helper — uncompleted task detection

- [ ] A task is uncompleted if marker is `Todo` or `InProgress`
- [ ] `Done` and `Cancelled` are completed
- [ ] For tree movement: the parent's status determines whether the tree moves

## 4. Helper — cascade cancellation

- [ ] When a parent is `Done`, walk children recursively
- [ ] Any child with `Todo` or `InProgress` marker → set to `Cancelled`
- [ ] Already `Done` or `Cancelled` children are unchanged

## 5. Section reordering after rollover

- [ ] Apply section sort: Today → Backlog → future dates (desc) → past dates (desc)
- [ ] Reuse the sort function from the renderer or share it

## 6. Tests

- [ ] Full rollover scenario from design doc (Feb 25 → Feb 26):
  - Demo moves to Today (due today)
  - API task tree carries forward (uncompleted parent)
  - Invoice moves to Backlog (overdue from older date)
  - Parser stays under date (completed)
  - Login bug stays, uncompleted subtask gets `[-]`
  - Old Today becomes `2026-02-25`
  - Report stays under future date
- [ ] No rollover when `last_rollover == today`
- [ ] Multi-day gap: tasks from multiple old date headings consolidate correctly
- [ ] Empty date sections removed after all tasks moved out
- [ ] Cascade: done parent marks `[ ]` and `[/]` children as `[-]`, leaves `[x]` alone
- [ ] Missing `last_rollover`: infers and proceeds

## Verify

- [ ] `cargo test -p tomb -- rollover` — all rollover tests pass
- [ ] Parse design doc example → rollover → render → compare to expected output
