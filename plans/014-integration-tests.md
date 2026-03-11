# Integration Tests

End-to-end tests covering the full CLI workflow.

Prerequisite: all command implementations (009 through 013)

## 1. Test infrastructure

- [ ] Use `tempfile::TempDir` for each test — isolated filesystem
- [ ] Helper: `run_tomb(args)` — invoke the CLI programmatically (call library functions directly, not subprocess)
- [ ] Helper: `setup_config(dir)` — create a `.tomb.toml` pointing to test files
- [ ] Helper: `read_file(path)` — read and return file contents for assertions

## 2. Init → Add → List round-trip

- [ ] `tomb init tasks.md --context work`
- [ ] `tomb add "First task" --due today`
- [ ] `tomb add "Second task"`
- [ ] `tomb list` → verify both tasks appear, first under Today, second under Backlog
- [ ] Verify IDs assigned to both tasks

## 3. Status transitions

- [ ] Add a task, get its ID
- [ ] `tomb start <id>` → verify marker is `[/]`
- [ ] `tomb done <id>` → verify marker is `[x]`
- [ ] Add task with subtasks (manual file edit), `tomb done <parent>` → verify children cascaded to `[-]`

## 4. Rollover behavior

- [ ] Create a managed file with `last_rollover` set to yesterday
- [ ] Add tasks under Today and a date heading for today
- [ ] Run any command (e.g., `tomb list`) to trigger rollover
- [ ] Verify: old Today became date heading, due-today tasks moved to new Today, old uncompleted carry forward

## 5. Sync

- [ ] Create file with tasks missing IDs (manual edit)
- [ ] Create file with duplicate IDs
- [ ] `tomb sync` → verify IDs assigned, duplicates fixed
- [ ] Run `tomb sync` again → verify idempotent (no changes)

## 6. Inbox capture

- [ ] `tomb inbox "Quick thought"` → verify appended to inbox file
- [ ] Verify task format: `- [ ] Quick thought`

## 7. ID prefix matching

- [ ] Add multiple tasks, get their IDs
- [ ] `tomb done <prefix>` with unique prefix → succeeds
- [ ] `tomb done <ambiguous-prefix>` → error listing matches
- [ ] `tomb done <nonexistent>` → error

## 8. Conflict detection

- [ ] Read a file, modify it externally (simulate concurrent edit), attempt write → verify `TombError::Conflict`

## 9. Multi-file operations

- [ ] Init two managed files with different contexts
- [ ] Add tasks to each
- [ ] `tomb list` → shows tasks from both files
- [ ] `tomb list --context work` → only work tasks
- [ ] `tomb done <id>` → finds task across files

## Verify

- [ ] `cargo test --test integration` — all integration tests pass
- [ ] Tests run in isolated temp directories, no filesystem side effects
