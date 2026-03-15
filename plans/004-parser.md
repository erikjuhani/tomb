# Markdown Parser

Implement `tomb-cli/src/parser.rs` — consume pulldown-cmark-task-marker (forked pulldown-cmark) events to build `ManagedFile` from markdown source.

Prerequisite: 002-tomb-parser.md (pulldown-cmark-task-marker fork compiles), 001-cargo-root.md (model types exist)

This is the most critical module. It must handle all the markdown constructs in the design doc: frontmatter, sections, tasks with nested subtasks, descriptions (including code blocks), IDs, and external links.

## 1. Parser function signature

- [ ] `pub fn parse_managed_file(input: &str) -> Result<ManagedFile>`
- [ ] Enable pulldown-cmark options: `ENABLE_TASKLISTS | ENABLE_EXTENDED_TASK_MARKERS | ENABLE_YAML_STYLE_METADATA_BLOCKS`
- [ ] Use `into_offset_iter()` to get `(Event, Range<usize>)` pairs

## 2. Frontmatter extraction

- [ ] Match `Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle))` → collect text → `Event::End`
- [ ] Parse YAML text for tomb fields: `tomb_mode`, `tomb_version`, `context`, `last_rollover`
- [ ] Simple key-value parsing (no need for a full YAML library — these are flat fields)
- [ ] Map to `Frontmatter` struct
- [ ] Handle missing frontmatter gracefully (return default `Frontmatter`)

## 3. Section detection

- [ ] Match `Event::Start(Tag::Heading { level: H2, .. })` → collect heading text
- [ ] Classify heading text:
  - `"Today"` → `SectionKind::Today`
  - `"Backlog"` → `SectionKind::Backlog`
  - Valid `YYYY-MM-DD` → `SectionKind::Date(NaiveDate)`
  - Anything else → skip or treat as non-section content
- [ ] Each section collects tasks until the next `## ` heading

## 4. Task parsing

- [ ] Match `Event::Start(Tag::Item)` for list items
- [ ] Match `Event::ExtendedTaskListMarker(char)` → map to `TaskMarker`:
  - `' '` → `Todo`, `'/'` → `InProgress`, `'x'`/`'X'` → `Done`, `'-'` → `Cancelled`
- [ ] Collect task title from subsequent `Text` events on the same line
- [ ] Extract block ID: trailing `^xxxx` pattern from title text (split off and store as `id`)
- [ ] Extract external link: `Event::Start(Tag::Link { dest_url, .. })` within the task line → store URL and link text
- [ ] Store source byte range from the offset iterator

## 5. Subtask nesting

- [ ] Track indentation level via `Event::Start(Tag::List(_))` / `Event::End(Tag::List(_))` nesting
- [ ] Build recursive `Task.children` tree from nested list items
- [ ] pulldown-cmark handles indentation detection — we just need to track start/end of nested lists

## 6. Description content

- [ ] After a task line, indented content that isn't a subtask is description
- [ ] Collect description lines as raw text (preserve formatting, code blocks, links)
- [ ] Handle code blocks within descriptions correctly (pulldown-cmark emits `CodeBlock` events — don't confuse with task content)
- [ ] Description belongs to the nearest preceding task at the same or higher indentation level

## 7. Mode resolution

- [ ] If frontmatter has `tomb_mode: managed` → `FileMode::Managed`
- [ ] If frontmatter has `tomb_mode: tracked` → `FileMode::Tracked`
- [ ] Eager inference: has `## Today` or `## Backlog` or date headings → `Managed`
- [ ] Fallback → `Tracked`

## 8. Tests

- [ ] Parse the design doc's managed file example (Today/Backlog/date sections with nested tasks)
- [ ] Frontmatter: parse all four fields, handle missing frontmatter
- [ ] Task markers: all four types (`[ ]`, `[/]`, `[x]`, `[-]`)
- [ ] Block IDs: extract `^xxxx` from task lines
- [ ] External links: extract URL and label from task lines
- [ ] Subtask nesting: 3 levels deep with descriptions at each level
- [ ] Code blocks in descriptions: don't break the parser
- [ ] Description with blank lines between paragraphs
- [ ] Mode inference: files with `## Today` infer as managed
- [ ] Round-trip: parse → check that source ranges allow surgical edits

## Verify

- [ ] `cargo test -p tomb-cli -- parser` — all parser tests pass
- [ ] Parse the full rollover example from the design doc without errors
