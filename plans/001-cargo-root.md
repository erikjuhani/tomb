# cargo-root: Tomb Crate Setup

Step-by-step tasks for populating the `tomb-cli` crate with all modules, CLI, error types, and data model.

The workspace and `tomb-cli` crate already exist. The `pulldown-cmark-task-marker` dependency is added later in plan 002 — this plan scaffolds everything else first so that `cargo check` passes without it.

## 1. Add MVP dependencies to `tomb-cli/Cargo.toml`

Add all dependencies except `pulldown-cmark-task-marker` (deferred to plan 002):

- [x] Add dependencies:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
rand = "0.9"
ratatui = "0.29"
crossterm = "0.28"
thiserror = "2"
etcetera = "0.8"

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
```

- [x] Verify: `cargo check -p tomb-cli` compiles (with stub source files from later steps)

## 2. Create module files with stub declarations

Create the source file tree under `tomb-cli/src/`. Each file starts as a stub with just enough to compile.

### 2a. `tomb-cli/src/lib.rs` — module declarations

- [ ] Create `tomb-cli/src/lib.rs` with `pub mod` for every module:

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

### 2b. `tomb-cli/src/error.rs` — TombError + Result alias

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

### 2c. `tomb-cli/src/model.rs` — core data types

- [ ] `TaskMarker` enum: `Todo`, `InProgress`, `Done`, `Cancelled`
- [ ] `TaskMarker` ↔ `char` conversion: `' '`, `'/'`, `'x'`, `'-'`
- [ ] `Task` struct: `id: Option<String>`, `title: String`, `marker: TaskMarker`, `description: Vec<String>`, `children: Vec<Task>`, `source_range: Option<Range<usize>>`
- [ ] `SectionKind` enum: `Today`, `Backlog`, `Date(NaiveDate)`
- [ ] `Section` struct: `kind: SectionKind`, `tasks: Vec<Task>`
- [ ] `Frontmatter` struct: `mode: Option<FileMode>`, `version: Option<u32>`, `context: Option<String>`, `last_rollover: Option<NaiveDate>`
- [ ] `FileMode` enum: `Managed`, `Tracked`
- [ ] `ManagedFile` struct: `frontmatter: Frontmatter`, `sections: Vec<Section>`, `source: String`

### 2d. `tomb-cli/src/cli.rs` — clap derive structs

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

### 2e. `tomb-cli/src/main.rs` — entry point

- [ ] Parse CLI args, dispatch to command handlers:

```rust
use clap::Parser;
use tomb_cli::{cli::{Cli, Commands}, error::Result};

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

- [ ] `tomb-cli/src/config.rs` — empty struct + `Config::resolve()` stub
- [ ] `tomb-cli/src/parser.rs` — `pub fn parse_managed_file(input: &str) -> Result<ManagedFile>` stub
- [ ] `tomb-cli/src/renderer.rs` — `pub fn render_managed_file(file: &ManagedFile) -> String` stub
- [ ] `tomb-cli/src/rollover.rs` — `pub fn maybe_rollover(file: &mut ManagedFile, today: NaiveDate) -> bool` stub
- [ ] `tomb-cli/src/id.rs` — `generate_id`, `resolve_prefix`, `assign_missing_ids` stubs
- [ ] `tomb-cli/src/io.rs` — `read_managed_file`, `write_managed_file` stubs

### 2g. `tomb-cli/src/commands/` — command module stubs

- [ ] Create `tomb-cli/src/commands/mod.rs` with re-exports
- [ ] Create stub files:
  - `tomb-cli/src/commands/init.rs`
  - `tomb-cli/src/commands/add.rs`
  - `tomb-cli/src/commands/status.rs` (done/start/cancel)
  - `tomb-cli/src/commands/list.rs`
  - `tomb-cli/src/commands/show.rs`
  - `tomb-cli/src/commands/sync.rs`
  - `tomb-cli/src/commands/inbox.rs`
  - `tomb-cli/src/commands/review.rs`
- [ ] Each file exports a `pub fn run(...) -> Result<()> { todo!() }`

## 3. Verify full skeleton

- [ ] `cargo build -p tomb-cli` — crate compiles (stubs only)
- [ ] `cargo run -p tomb-cli -- --help` — shows all subcommands with descriptions
- [ ] `cargo run -p tomb-cli -- init foo.md` — panics with `todo!()` (expected)
- [ ] `cargo test -p tomb-cli` — passes (no tests yet, but no compile errors)
