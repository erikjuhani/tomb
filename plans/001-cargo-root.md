# cargo-root: Tomb Crate Setup

Set up the `tomb-cli` crate foundation: dependencies, error types, core data model, and CLI entry point. Each task is a vertical slice that keeps the crate compiling.

The workspace and `tomb-cli` crate already exist with dependencies in place. The `pulldown-cmark-task-marker` dependency is added later in plan 002.

## 1. Add MVP dependencies to `tomb-cli/Cargo.toml`

- [x] Add dependencies
- [x] Verify: `cargo check -p tomb-cli` compiles

## 2. Error types

Add `error.rs` with `TombError` and the `Result` alias. Only include error variants that have a consumer right now — start minimal and grow the enum as later tasks need new variants.

- [x] Create `tomb-cli/src/error.rs` with `TombError` enum (`Io` variant to start) and `pub type Result<T>`
- [x] Create `tomb-cli/src/lib.rs` declaring `pub mod error;`
- [x] Verify: `cargo check -p tomb-cli`

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

Add `model.rs` with the core data structures that the rest of the crate operates on.

- [x] Create `tomb-cli/src/model.rs` with:
  - `TaskMarker` enum (`Todo`, `InProgress`, `Done`, `Cancelled`)
  - `Task` struct: `id: Option<String>`, `title: String`, `marker: TaskMarker`, `description: Vec<String>`, `children: Vec<Task>`, `source_range: Option<Range<usize>>`
  - `SectionKind` enum: `Today`, `Backlog`, `Date(NaiveDate)`
  - `Section` struct: `kind: SectionKind`, `tasks: Vec<Task>`
  - `FileMode` enum: `Managed`, `Tracked`
  - `Frontmatter` struct: `mode: Option<FileMode>`, `version: Option<u32>`, `context: Option<String>`, `last_rollover: Option<NaiveDate>`
  - `ManagedFile` struct: `frontmatter: Frontmatter`, `sections: Vec<Section>`, `source: String`
- [x] Add `pub mod model;` to `lib.rs`
- [x] Verify: `cargo check -p tomb-cli`

## 4. CLI parsing and entry point

Add `cli.rs` with clap derive structs and wire up `main.rs` to parse args and dispatch. Every match arm uses `todo!()` for now.

- [ ] Create `tomb-cli/src/cli.rs` with `Cli` and `Commands` enum:

```rust
#[derive(Parser)]
#[command(name = "tomb", about = "Markdown task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init { path: PathBuf, #[arg(long)] context: Option<String> },
    Add { text: String, #[arg(long)] due: Option<String>, #[arg(long)] context: Option<String> },
    Done { id: String },
    Start { id: String },
    Cancel { id: String },
    List { #[arg(long)] due: Option<String>, #[arg(long)] context: Option<String> },
    Show { id: String },
    Sync,
    Inbox { text: Option<String> },
    Review,
}
```

- [ ] Update `tomb-cli/src/main.rs` to parse CLI and match on commands (all arms `todo!()`)
- [ ] Add `pub mod cli;` to `lib.rs`
- [ ] Verify: `cargo check -p tomb-cli`
- [ ] Verify: `cargo run -p tomb-cli -- --help` shows subcommands
