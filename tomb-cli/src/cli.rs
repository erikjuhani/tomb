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

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use crate::{cli::Cli, version};

    #[test]
    fn version_output() {
        let help = Cli::command()
            .version(
                version::VersionInfo {
                    version: "0.1.0",
                    short_hash: "abc123def",
                    date: "2026-03-15",
                }
                .to_string(),
            )
            .render_version()
            .to_string();
        insta::assert_snapshot!(help)
    }

    #[test]
    fn help_output() {
        let help = Cli::command().render_help().to_string();
        insta::assert_snapshot!(help)
    }
}
