# pulldown-cmark-task-marker: Forked pulldown-cmark Sub-crate Setup

Step-by-step tasks for adding pulldown-cmark as a git subtree workspace member and applying our custom task marker extension.

## 1. Add pulldown-cmark as git subtree

```bash
git subtree add --prefix=crates/pulldown-cmark-task-marker https://github.com/raphlinus/pulldown-cmark.git v0.13.1 --squash
```

This imports the v0.13.1 tag into `crates/pulldown-cmark-task-marker/` with a single squashed commit.

## 2. Convert root Cargo.toml to workspace

Update the root `Cargo.toml` to declare a workspace:

```toml
[workspace]
members = ["crates/pulldown-cmark-task-marker"]

[package]
name = "tomb"
version = "0.1.0"
edition = "2021"

[dependencies]
pulldown-cmark-task-marker = { path = "crates/pulldown-cmark-task-marker", default-features = false }
```

## 3. Strip the fork to library-only

Remove non-library artifacts from `crates/pulldown-cmark-task-marker/`:

- [ ] Delete `crates/pulldown-cmark-task-marker/examples/`
- [ ] Delete `crates/pulldown-cmark-task-marker/benches/`
- [ ] Delete `crates/pulldown-cmark-task-marker/tests/` (or keep for regression)
- [ ] Delete `crates/pulldown-cmark-task-marker/fuzz/`
- [ ] Remove the `[[bin]]` section from `crates/pulldown-cmark-task-marker/Cargo.toml` (the `pulldown-cmark` binary)
- [ ] Remove `getopts` and `pulldown-cmark-escape` from dependencies
- [ ] Set `default-features = false` — strip `html` and `getopts` features
- [ ] Rename package to `pulldown-cmark-task-marker` in `crates/pulldown-cmark-task-marker/Cargo.toml`
- [ ] Verify: `cargo build -p pulldown-cmark-task-marker` compiles

## 4. Apply fork changes — add `ExtendedTaskListMarker(char)` variant + new option

The key design: the existing `Event::TaskListMarker(bool)` is **untouched**. A new variant `Event::ExtendedTaskListMarker(char)` is emitted instead when `ENABLE_EXTENDED_TASK_MARKERS` is active. This is fully non-breaking — existing code matching on `TaskListMarker(bool)` compiles and works identically.

| Option state | Scanner accepts | Emits |
|---|---|---|
| `ENABLE_TASKLISTS` only (default) | `x`, `X`, space | `TaskListMarker(bool)` — unchanged upstream behavior |
| `ENABLE_TASKLISTS + ENABLE_EXTENDED_TASK_MARKERS` | Any char except `]` | `ExtendedTaskListMarker(char)` — `'x'`, `' '`, `'/'`, `'-'`, etc. |

When `ENABLE_EXTENDED_TASK_MARKERS` is on, `TaskListMarker(bool)` is never emitted — the extended variant fully replaces it. This means consumers only need to match on one or the other, not both.

### 4a. `src/lib.rs` — Options bitflag + new Event variant

- [ ] Add `ENABLE_EXTENDED_TASK_MARKERS` to the `Options` bitflags (next available bit)
- [ ] Add `ExtendedTaskListMarker(char)` variant to `Event` enum (keep `TaskListMarker(bool)` as-is)
- [ ] Add `into_static()` match arm for the new variant

### 4b. `src/scanners.rs` — scan_task_list_marker

- [ ] Add new method `scan_extended_task_list_marker` returning `Option<char>`
- [ ] Accepts any single character except `]`
- [ ] Keep existing `scan_task_list_marker` returning `Option<bool>` unchanged

```rust
// Existing (unchanged):
pub(crate) fn scan_task_list_marker(&mut self) -> Option<bool> {
    // ... only x/X/space
    Some(is_checked)
}

// New:
pub(crate) fn scan_extended_task_list_marker(&mut self) -> Option<char> {
    // ...
    let marker = match self.bytes.get(self.ix) {
        Some(&c) if c != b']' => { self.ix += 1; c as char }
        _ => { *self = save; return None; }
    };
    // ...
    Some(marker)
}
```

### 4c. `src/parse.rs` — ItemBody enum + event emission

- [ ] Add `ItemBody::ExtendedTaskListMarker(char)` variant (keep `TaskListMarker(bool)` as-is)
- [ ] Check `options.contains(Options::ENABLE_EXTENDED_TASK_MARKERS)`:
  - If true: call `scan_extended_task_list_marker`, store as `ExtendedTaskListMarker(char)`
  - If false: call `scan_task_list_marker` as before, store as `TaskListMarker(bool)`
- [ ] Event emission: emit `Event::ExtendedTaskListMarker(char)` or `Event::TaskListMarker(bool)` accordingly

### 4d. `src/firstpass.rs` — plumb new variant through

- [ ] Handle `ExtendedTaskListMarker(char)` alongside existing `TaskListMarker(bool)` in relevant match arms
- [ ] Call the appropriate scanner based on options

### 4e. `src/html.rs` — HTML rendering for new variant

- [ ] Add match arm for `ExtendedTaskListMarker(char)`: `' '` → unchecked, anything else → checked
- [ ] Existing `TaskListMarker(bool)` rendering unchanged

### 4f. Verify

- [ ] `cargo build -p pulldown-cmark-task-marker` compiles clean
- [ ] Test: default options — `- [/]` does NOT parse as task (standard GFM)
- [ ] Test: default options — `- [x]` and `- [ ]` parse as `TaskListMarker(true)` and `TaskListMarker(false)` (unchanged)
- [ ] Test: with `ENABLE_EXTENDED_TASK_MARKERS` — `- [x]` emits `ExtendedTaskListMarker('x')`, `- [ ]` emits `ExtendedTaskListMarker(' ')`
- [ ] Test: with `ENABLE_EXTENDED_TASK_MARKERS` — `- [/]`, `- [-]`, `- [!]`, `- [?]` emit `ExtendedTaskListMarker` with correct chars
- [ ] Test: `- []` (empty) does not parse as task in either mode
- [ ] Test: `- [ab]` (multi-char) does not parse as task in either mode

## 5. Wire tomb crate to use pulldown-cmark-task-marker

- [ ] Replace `pulldown-cmark` dependency with `pulldown-cmark-task-marker` in root `Cargo.toml`
- [ ] Update imports in `src/` from `pulldown_cmark::` to `pulldown_cmark_task_marker::`
- [ ] Enable `ENABLE_TASKLISTS | ENABLE_EXTENDED_TASK_MARKERS` in tomb's parser
- [ ] `cargo build` — full workspace compiles

## 6. Future: pulling upstream updates

When pulldown-cmark releases a new version:

```bash
git subtree pull --prefix=crates/pulldown-cmark-task-marker https://github.com/raphlinus/pulldown-cmark.git <new-tag> --squash
```

Conflicts will only arise if upstream changes the task list marker code path — unlikely since our change is a small type widening in an isolated scanner function.
