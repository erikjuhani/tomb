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

Add `cli.rs` with clap derive structs and wire up `main.rs` to parse args and dispatch. Every match arm uses `todo!()` for now. Include `--version` from day one using cargo's version format.

- [x] Create `tomb-cli/build.rs` to inject git commit info at compile time
- [x] Add `[[bin]]` section to `Cargo.toml` naming the binary `tomb`

- [x] Create `tomb-cli/src/cli.rs` with `Cli` and `Commands` enum. Use `Cli::command().version()` at runtime to set the version string since `format!` can't be used in derive attributes:

```rust
use std::path::PathBuf;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(about)]
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

pub fn version_string() -> String {
    format!(
        "{} ({} {})",
        env!("TOMB_VERSION"),
        env!("TOMB_COMMIT_SHORT_HASH"),
        env!("TOMB_COMMIT_DATE"),
    )
}
```

- [x] Update `tomb-cli/src/main.rs` to parse CLI with version and match on commands (all arms `todo!()`):

```rust
use clap::{CommandFactory, Parser};
use tomb_cli::cli::{self, Cli, Commands};
use tomb_cli::error::Result;

fn main() -> Result<()> {
    let cli = Cli::try_parse_from(
        Cli::command().version(cli::version_string()).get_matches()
    );
    match cli.command {
        Commands::Init { .. } => todo!(),
        Commands::Add { .. } => todo!(),
        Commands::Done { .. } => todo!(),
        Commands::Start { .. } => todo!(),
        Commands::Cancel { .. } => todo!(),
        Commands::List { .. } => todo!(),
        Commands::Show { .. } => todo!(),
        Commands::Sync => todo!(),
        Commands::Inbox { .. } => todo!(),
        Commands::Review => todo!(),
    }
}
```

- [x] Add `pub mod cli;` to `lib.rs`
- [x] Verify: `cargo check -p tomb-cli`
- [x] Verify: `cargo run -p tomb-cli -- --help` shows subcommands
- [x] Verify: `cargo run -p tomb-cli -- --version` shows version with commit hash and date

- [x] Add `insta` to dev-dependencies in `tomb-cli/Cargo.toml`
- [x] Add snapshot tests for help output using `insta`:

```rust
use clap::CommandFactory;
use tomb_cli::cli::{self, Cli};

#[test]
fn help_output() {
    let help = Cli::command().version(cli::version_string()).render_help().to_string();
    insta::assert_snapshot!(help);
}
```
