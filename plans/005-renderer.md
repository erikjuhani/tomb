# Markdown Renderer

Implement `tomb-cli/src/renderer.rs` — serialize `ManagedFile` back to a markdown string.

Prerequisite: 001-cargo-root.md (model types), 004-parser.md (parser exists for round-trip testing)

## 1. Module scaffolding

- [ ] Create `tomb-cli/src/renderer.rs`
- [ ] Add `pub mod renderer;` to `lib.rs`
- [ ] Verify: `cargo check -p tomb-cli`

## 2. Render function

- [ ] `pub fn render_managed_file(file: &ManagedFile) -> String`
- [ ] Build output string by walking the `ManagedFile` structure

## 3. Frontmatter rendering

- [ ] Emit `---\n` delimiters
- [ ] Write fields in order: `tomb_mode`, `tomb_version`, `context`, `last_rollover`
- [ ] Only emit fields that are `Some`
- [ ] Format `last_rollover` as `YYYY-MM-DD`

## 4. Section rendering

- [ ] Emit `## {heading}\n\n` for each section
- [ ] Section heading text: `"Today"`, `"Backlog"`, or `YYYY-MM-DD` for date sections
- [ ] Section sort order: Today → Backlog → future dates (desc) → past dates (desc)

## 5. Task rendering

- [ ] `- [{marker}] {title}` with correct marker char
- [ ] Append external link before block ID: `[label](url)`
- [ ] Append block ID: ` ^{id}`
- [ ] Indent subtasks: 2 spaces per nesting level
- [ ] Recurse into `children` with increased indent

## 6. Description rendering

- [ ] Emit description lines indented to match their parent task
- [ ] Preserve blank lines within multi-paragraph descriptions
- [ ] Preserve code blocks, links, and formatting verbatim

## 7. Spacing

- [ ] Blank line between sections
- [ ] Blank line after frontmatter
- [ ] Consistent trailing newline at end of file
- [ ] No trailing whitespace on lines

## 8. Tests

- [ ] Round-trip: parse example → render → compare to original (byte-for-byte where possible)
- [ ] Frontmatter with all fields, with partial fields, with no frontmatter
- [ ] Section ordering: verify Today before Backlog before dates
- [ ] Nested tasks at 3 levels with descriptions
- [ ] Code blocks in descriptions preserved
- [ ] External links rendered correctly
- [ ] Empty sections (no tasks) still render with heading

## Verify

- [ ] `cargo test -p tomb-cli -- renderer` — all renderer tests pass
- [ ] Round-trip the design doc's full rollover example: parse → render → parse again → assert equal model
