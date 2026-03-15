use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::version;

const VERSION_INFO: version::VersionInfo = version::VersionInfo::from_env();

#[derive(Parser)]
#[command(name = "tomb", version = VERSION_INFO.to_string())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        path: PathBuf,
        #[arg(long)]
        context: Option<String>,
    },
    Add {
        text: String,
        #[arg(long)]
        due: Option<String>,
        #[arg(long)]
        context: Option<String>,
    },
    Done {
        id: String,
    },
    Start {
        id: String,
    },
    Cancel {
        id: String,
    },
    List {
        #[arg(long)]
        due: Option<String>,
        #[arg(long)]
        context: Option<String>,
    },
    Show {
        id: String,
    },
    Sync,
    Inbox {
        text: Option<String>,
    },
    Review,
}

pub fn version_string() -> String {
    format!(
        "{} ({} {})",
        env!("TOMB_VERSION"),
        env!("TOMB_COMMIT_SHORT_HASH"),
        env!("TOMB_COMMIT_DATE")
    )
}
