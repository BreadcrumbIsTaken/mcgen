//! # mcgen
//! ## Generate a Minecraft server in seconds!
mod adding;
mod cli;
mod config;
mod consts;
mod downloading;
mod gen;
mod updating;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use dirs::config_dir;
use std::path::Path;

use crate::{
    adding::add_plugin_to_existing_server, config::Config, gen::generation::generate_server,
    updating::update,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config_path = format!("{}\\mcgen", config_dir().unwrap().display());
    let mut config = Config::new(Path::new(&config_path));
    config.init_config().unwrap_or_else(|err| {
        eprintln!("{} {}", "Error initiating config! Error:".red(), err);
        std::process::exit(1);
    });

    match cli.commands {
        Commands::Gen {
            dir,
            bungeecord,
            aikars_flags,
        } => {
            // yeah i think this looks funny too but it lets the terminal use colors :D
            println!("{} {}/", "Creating a new server in directory:".green(), dir);
            println!("---");

            generate_server(&dir, bungeecord, aikars_flags, &config).await;
        }
        Commands::Update { directories, check } => {
            update(directories, check).await.unwrap_or_else(|err| {
                eprintln!(
                    "{} {}",
                    "Error updating server and/or plugins! Error:".red(),
                    err
                )
            })
        }
        Commands::Add { directory, name, url } => {
            add_plugin_to_existing_server(directory, name, url)
                .await
                .unwrap_or_else(|err| eprintln!("{} {}", "Error adding plugin! Error:".red(), err));
        }
        Commands::Config {} => {
            config.open_config().unwrap_or_else(|err| {
                eprintln!("{} {}", "Error opening config! Error:".red(), err)
            });
        }
    }
}
