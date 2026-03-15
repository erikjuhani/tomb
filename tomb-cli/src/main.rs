use clap::{CommandFactory, FromArgMatches};
use tomb_cli::{
    cli::{Cli, Commands},
    error::Result,
};

fn main() -> Result<()> {
    let matches = Cli::command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

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
