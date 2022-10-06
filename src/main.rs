mod cli;
mod fetching;
mod sources;
mod commands;
mod settings;

use clap::Parser;
use cli::{Cli, Commands};
use commands::gen::begin_gen;
use settings::GenSettings;
use sources::{
    jenkins::JenkinsSource,
    registry::SourceRegistry,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut registry = SourceRegistry::new();
    registry.register_source(JenkinsSource::new("jenkins".to_string()));
    registry.run_source("jenkins".to_string());

    match cli.commands {
        Commands::Gen {
            dir,
            bungeecord,
            aikars_flags,
            accept_eula,
            dont_generate_start_scripts,
            version,
        } => {
            let mut builder = GenSettings::new();
            builder.set_directory(dir);
            builder.set_bungeecord(bungeecord);
            builder.set_aikars_flags(aikars_flags);
            builder.set_accept_eula(accept_eula);
            builder.set_dont_gen_start_scripts(dont_generate_start_scripts);
            builder.set_version(version);

            begin_gen(builder);
        },
        Commands::Update {
            directories,
            check,
            dont_update_version,
        } => todo!(),
        Commands::Add(_) => todo!(),
        Commands::Config { regenerate } => todo!(),
    }
}
