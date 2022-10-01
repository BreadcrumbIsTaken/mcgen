mod cli;
mod sources;

use clap::Parser;
use cli::{Cli, Commands};
use sources::registry::SourceRegistry;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut registry = SourceRegistry::new();

    match cli.commands {
        Commands::Gen {
            dir,
            bungeecord,
            aikars_flags,
            accept_eula,
            dont_generate_start_scripts,
            version,
        } => todo!(),
        Commands::Update {
            directories,
            check,
            dont_update_version,
        } => todo!(),
        Commands::Add(_) => todo!(),
        Commands::Config { regenerate } => todo!(),
    }
}
