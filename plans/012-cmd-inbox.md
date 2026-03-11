# Command: inbox

Implement `src/commands/inbox.rs`.

Prerequisite: 003-config.md

## 1. `tomb inbox <text>` — quick capture

- [ ] Resolve nearest inbox file via `Config::nearest_inbox(cwd)`
- [ ] Error if no inbox configured
- [ ] Append `- [ ] {text}\n` to end of file
- [ ] Create the file if it doesn't exist
- [ ] Print: `Added to {inbox_path}`

## 2. `tomb inbox` — open in editor

- [ ] Resolve nearest inbox file
- [ ] Read `$EDITOR` env var (fall back to `vi`)
- [ ] Spawn editor as child process, wait for exit
- [ ] No post-processing needed — user edits freely

## 3. Wire into CLI dispatch

- [ ] Update `main.rs` match arm for `Inbox`
- [ ] Branch on `text: Some(_)` vs `None`

## 4. Tests

- [ ] Quick capture appends task line to existing inbox
- [ ] Quick capture creates inbox file if missing
- [ ] Appended line is a valid task format (`- [ ] {text}\n`)
- [ ] Nearest inbox resolution: per-repo inbox wins over global
- [ ] Error when no inbox configured

## Verify

- [ ] `cargo test -p tomb -- inbox`
- [ ] Manual: `tomb inbox "Quick thought"` → check file → `tomb inbox` → editor opens
