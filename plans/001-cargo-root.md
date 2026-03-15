# cargo-root: Tomb Crate Setup

Incrementally build out the `tomb-cli` crate module by module. Each task is a vertical slice that adds one cohesive piece, keeps the crate compiling, and results in a single focused commit.

The workspace and `tomb-cli` crate already exist with dependencies in place. The `pulldown-cmark-task-marker` dependency is added later in plan 002.

## 1. Add MVP dependencies to `tomb-cli/Cargo.toml`

- [x] Add dependencies
- [x] Verify: `cargo check -p tomb-cli` compiles

## 2. Error types

Add `error.rs` with `TombError` and the `Result` alias. Only include error variants that have a consumer right now — start minimal and grow the enum as later tasks need new variants.

- [ ] Create `tomb-cli/src/error.rs` with `TombError` enum (`Io` variant to start) and `pub type Result<T>`
- [ ] Create `tomb-cli/src/lib.rs` declaring `pub mod error;`
- [ ] Verify: `cargo check -p tomb-cli`

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TombError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TombError>;
```

## 3. Model types

Add `model.rs` with the core data structures that the rest of the crate operates on. Add error variants to `TombError` only if model code needs them.

- [ ] Create `tomb-cli/src/model.rs` with:
  - `TaskMarker` enum (`Todo`, `InProgress`, `Done`, `Cancelled`) with `char` conversions
  - `Task` struct
  - `SectionKind` enum (`Today`, `Backlog`, `Date(NaiveDate)`)
  - `Section` struct
  - `FileMode` enum (`Managed`, `Tracked`)
  - `Frontmatter` struct
  - `ManagedFile` struct
- [ ] Add `pub mod model;` to `lib.rs`
- [ ] Verify: `cargo check -p tomb-cli`

## 4. CLI parsing and entry point

Add `cli.rs` with clap derive structs and wire up `main.rs` to parse args and dispatch. Every match arm uses `todo!()` for now.

- [ ] Create `tomb-cli/src/cli.rs` with `Cli` and `Commands` enum
- [ ] Update `tomb-cli/src/main.rs` to parse CLI and match on commands
- [ ] Add `pub mod cli;` to `lib.rs`
- [ ] Verify: `cargo check -p tomb-cli`
- [ ] Verify: `cargo run -p tomb-cli -- --help` shows subcommands

## 5. Config

Add `config.rs` with config resolution logic. This is where tomb looks for its configuration (managed files, context settings).

- [ ] Create `tomb-cli/src/config.rs` with `Config` struct and `Config::resolve()` stub
- [ ] Add `pub mod config;` to `lib.rs`
- [ ] Add `Config` error variant to `TombError` if needed
- [ ] Verify: `cargo check -p tomb-cli`

## 6. ID generation and resolution

Add `id.rs` with functions for generating short IDs and resolving prefix matches against a list of tasks.

- [ ] Create `tomb-cli/src/id.rs` with `generate_id`, `resolve_prefix`, `assign_missing_ids`
- [ ] Add `pub mod id;` to `lib.rs`
- [ ] Add `IdNotFound` and `AmbiguousId` error variants to `TombError`
- [ ] Verify: `cargo check -p tomb-cli`

## 7. Parser and renderer

Add `parser.rs` and `renderer.rs` together since they are inverses of each other and share model types. The parser turns markdown text into a `ManagedFile`, the renderer turns it back.

- [ ] Create `tomb-cli/src/parser.rs` with `pub fn parse_managed_file(input: &str) -> Result<ManagedFile>`
- [ ] Create `tomb-cli/src/renderer.rs` with `pub fn render_managed_file(file: &ManagedFile) -> String`
- [ ] Add `pub mod parser;` and `pub mod renderer;` to `lib.rs`
- [ ] Add `Parse`, `UnsupportedVersion` error variants to `TombError`
- [ ] Verify: `cargo check -p tomb-cli`

## 8. File I/O

Add `io.rs` for reading and writing managed files on disk. Depends on parser and renderer.

- [ ] Create `tomb-cli/src/io.rs` with `read_managed_file`, `write_managed_file`
- [ ] Add `pub mod io;` to `lib.rs`
- [ ] Add `NotManaged`, `Conflict` error variants to `TombError` if needed
- [ ] Verify: `cargo check -p tomb-cli`

## 9. Rollover

Add `rollover.rs` for date-based task rollover logic.

- [ ] Create `tomb-cli/src/rollover.rs` with `pub fn maybe_rollover(file: &mut ManagedFile, today: NaiveDate) -> bool`
- [ ] Add `pub mod rollover;` to `lib.rs`
- [ ] Verify: `cargo check -p tomb-cli`

## 10. Command modules

Add the `commands/` directory with one module per command (or group of related commands). Wire each into the `main.rs` match. Each command function takes the parsed args and returns `Result<()>`.

- [ ] Create `tomb-cli/src/commands/mod.rs` with submodule declarations
- [ ] Create command files, each exporting `pub fn run(...) -> Result<()> { todo!() }`:
  - `commands/init.rs`
  - `commands/add.rs`
  - `commands/status.rs` (handles done/start/cancel)
  - `commands/list.rs`
  - `commands/show.rs`
  - `commands/sync.rs`
  - `commands/inbox.rs`
  - `commands/review.rs`
- [ ] Add `pub mod commands;` to `lib.rs`
- [ ] Update `main.rs` match arms to call command functions instead of `todo!()`
- [ ] Verify: `cargo build -p tomb-cli`
- [ ] Verify: `cargo run -p tomb-cli -- --help` shows all subcommands
- [ ] Verify: `cargo test -p tomb-cli` passes
