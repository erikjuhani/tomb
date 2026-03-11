# cargo-root: Workspace & Tomb Crate Setup

Step-by-step tasks for converting the repo to a Cargo workspace and scaffolding the `tomb` binary crate with all modules, CLI, error types, and data model.

Prerequisite: `tomb-parser.md` (subtree import) should be done first so `crates/pulldown-cmark-task-marker/` exists.

## 1. Convert root Cargo.toml to workspace

Replace the current `Cargo.toml` with a workspace-aware manifest:

- [ ] Add `[workspace]` section with `members = ["crates/pulldown-cmark-task-marker"]`
- [ ] Keep `[package]` for the root `tomb` binary crate
- [ ] Replace `pulldown-cmark` dependency with `pulldown-cmark-task-marker = { path = "crates/pulldown-cmark-task-marker", default-features = false }`
- [ ] Add all MVP dependencies:

```toml
[workspace]
members = ["crates/pulldown-cmark-task-marker"]

[package]
name = "tomb"
version = "0.1.0"
edition = "2021"

[dependencies]
pulldown-cmark-task-marker = { path = "crates/pulldown-cmark-task-marker", default-features = false }
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
rand = "0.9"
ratatui = "0.29"
crossterm = "0.28"
thiserror = "2"
dirs = "6"
fs2 = "0.4"

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
```

- [ ] Verify: `cargo check` compiles (with stub source files from later steps)

## 2. Create module files with stub declarations

Create the source file tree. Each file starts as a stub with just enough to compile.

### 2a. `src/lib.rs` — module declarations

- [ ] Create `src/lib.rs` with `pub mod` for every module:

```rust
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod id;
pub mod io;
pub mod model;
pub mod parser;
pub mod renderer;
pub mod rollover;
```

### 2b. `src/error.rs` — TombError + Result alias

- [ ] Define `TombError` enum using `thiserror`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TombError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("ID not found: {0}")]
    IdNotFound(String),
    #[error("Ambiguous ID prefix '{0}': matches {1:?}")]
    AmbiguousId(String, Vec<String>),
    #[error("File conflict: {0}")]
    Conflict(String),
    #[error("Not a managed file: {0}")]
    NotManaged(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),
}

pub type Result<T> = std::result::Result<T, TombError>;
```

### 2c. `src/model.rs` — core data types

- [ ] `TaskMarker` enum: `Todo`, `InProgress`, `Done`, `Cancelled`
- [ ] `TaskMarker` ↔ `char` conversion: `' '`, `'/'`, `'x'`, `'-'`
- [ ] `Task` struct: `id: Option<String>`, `title: String`, `marker: TaskMarker`, `description: Vec<String>`, `children: Vec<Task>`, `source_range: Option<Range<usize>>`
- [ ] `SectionKind` enum: `Today`, `Backlog`, `Date(NaiveDate)`
- [ ] `Section` struct: `kind: SectionKind`, `tasks: Vec<Task>`
- [ ] `Frontmatter` struct: `mode: Option<FileMode>`, `version: Option<u32>`, `context: Option<String>`, `last_rollover: Option<NaiveDate>`
- [ ] `FileMode` enum: `Managed`, `Tracked`
- [ ] `ManagedFile` struct: `frontmatter: Frontmatter`, `sections: Vec<Section>`, `source: String`

### 2d. `src/cli.rs` — clap derive structs

- [ ] `Cli` struct with `#[command]` and `Commands` subcommand enum
- [ ] Subcommands matching MVP scope:

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

### 2e. `src/main.rs` — entry point

- [ ] Parse CLI args, dispatch to command handlers:

```rust
use clap::Parser;
use tomb::{cli::{Cli, Commands}, error::Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { .. } => todo!(),
        Commands::Add { .. } => todo!(),
        // ... etc
    }
}
```

### 2f. Remaining module stubs

Each file is created empty or with a placeholder function:

- [ ] `src/config.rs` — empty struct + `Config::resolve()` stub
- [ ] `src/parser.rs` — `pub fn parse_managed_file(input: &str) -> Result<ManagedFile>` stub
- [ ] `src/renderer.rs` — `pub fn render_managed_file(file: &ManagedFile) -> String` stub
- [ ] `src/rollover.rs` — `pub fn maybe_rollover(file: &mut ManagedFile, today: NaiveDate) -> bool` stub
- [ ] `src/id.rs` — `generate_id`, `resolve_prefix`, `assign_missing_ids` stubs
- [ ] `src/io.rs` — `read_managed_file`, `write_managed_file` stubs

### 2g. `src/commands/` — command module stubs

- [ ] Create `src/commands/mod.rs` with re-exports
- [ ] Create stub files:
  - `src/commands/init.rs`
  - `src/commands/add.rs`
  - `src/commands/status.rs` (done/start/cancel)
  - `src/commands/list.rs`
  - `src/commands/show.rs`
  - `src/commands/sync.rs`
  - `src/commands/inbox.rs`
  - `src/commands/review.rs`
- [ ] Each file exports a `pub fn run(...) -> Result<()> { todo!() }`

## 3. Clean up old files

- [ ] Remove `examples/` directory (v1 parser experiments, superseded by pulldown-cmark-task-marker fork)
- [ ] Remove old `pulldown-cmark` dependency from Cargo.toml (replaced by pulldown-cmark-task-marker)

## 4. Verify full skeleton

- [ ] `cargo build` — workspace compiles (stubs + pulldown-cmark-task-marker)
- [ ] `cargo run -- --help` — shows all subcommands with descriptions
- [ ] `cargo run -- init foo.md` — panics with `todo!()` (expected)
- [ ] `cargo test` — passes (no tests yet, but no compile errors)
- [ ] `cargo build -p pulldown-cmark-task-marker` — sub-crate builds independently
