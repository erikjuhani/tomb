# Command: review (ratatui TUI)

Implement `tomb-cli/src/commands/review.rs` — interactive inbox/backlog triage.

Prerequisite: 009-cmd-init-sync.md (commands/ module exists), 012-cmd-inbox.md, 010-cmd-add-status.md, ratatui + crossterm dependencies

This is the most complex command — a full TUI with keyboard-driven workflow.

## 1. Module scaffolding

- [ ] Add `tomb-cli/src/commands/review.rs`
- [ ] Add `pub mod review;` to `commands/mod.rs`
- [ ] Verify: `cargo check -p tomb-cli`

## 2. Inbox parsing

- [ ] Read inbox file contents
- [ ] Classify each item:
  - Lines matching `- [ ] ...` → task items
  - Non-empty lines/paragraphs that aren't tasks → plain text items
  - Skip blank lines (they separate items)
- [ ] Build a `Vec<InboxItem>` with `enum InboxItem { Task(String), Text(String) }`

## 3. TUI setup

- [ ] Enter ratatui alternate screen (`crossterm::terminal::enable_raw_mode`, `EnterAlternateScreen`)
- [ ] Create `Terminal<CrosstermBackend<Stdout>>`
- [ ] Set up panic hook to restore terminal on crash

## 4. Review loop — inbox mode

- [ ] State: current item index, list of items, list of actions taken
- [ ] Display:
  - Item counter: `{n}/{total}`
  - Current item text
  - Item type indicator: `(task)` or `(plain text)`
  - Action keys: `[a]dd to file  [s]kip  [d]elete  [e]dit  [q]uit`
- [ ] Key handlers:
  - `a` → add flow: prompt for context (list available), prompt for due date (optional), add task to managed file via `commands::add` logic
  - `s` → mark as skipped, advance
  - `d` → mark as deleted, advance
  - `e` → edit item text inline (simple line edit)
  - `q` → exit review early

## 5. Context and date prompts

- [ ] After pressing `a`, show context selection (list contexts from config)
- [ ] Accept typed input or number selection
- [ ] Then prompt for due date: accept `today`, `tomorrow`, `YYYY-MM-DD`, or Enter to skip (→ Backlog)
- [ ] Add the task and show confirmation inline

## 6. Post-review cleanup

- [ ] Rewrite inbox file with only skipped/unprocessed items
- [ ] Deleted and added items are removed
- [ ] Print summary: `Review complete. {n} items processed, {skipped} skipped.`

## 7. Terminal restoration

- [ ] On normal exit, quit, or panic: restore terminal (disable raw mode, leave alternate screen)
- [ ] Use `scopeguard` or `Drop` impl for safety

## 8. Tests

- [ ] Inbox parsing: tasks and text items classified correctly
- [ ] Inbox rewrite: only skipped items remain
- [ ] Add action creates task in target file
- [ ] Delete removes item from inbox
- [ ] TUI rendering (snapshot tests or manual verification)

## Verify

- [ ] `cargo test -p tomb-cli -- review`
- [ ] Manual: populate inbox → `tomb review` → process items → verify inbox shrinks and tasks appear in managed files
