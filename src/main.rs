//! # mcgen
//! ## Generate a Minecraft server in seconds!
#![allow(clippy::too_many_arguments)]
mod adding;
mod cli;
mod config;
mod consts;
mod downloading;
mod gen;
mod updating;

use clap::Parser;
use cli::{AddCommands, Cli, Commands};
use colored::*;
use dirs::config_dir;
use gen::start_script::generate_start_script_paper;
use std::path::Path;

use crate::{
    adding::add_plugin_to_existing_server, config::Config, gen::generate_server, updating::update,
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
            accept_eula,
            dont_generate_start_scripts,
            version,
        } => {
            // yeah i think this looks funny too but it lets the terminal use colors :D
            println!("{} {}", "Creating a new server in directory:".green(), dir);
            println!("---");

            generate_server(
                &dir,
                bungeecord,
                aikars_flags,
                accept_eula,
                dont_generate_start_scripts,
                false,
                false,
                false,
                false,
                version,
                &config,
            )
            .await;
        }
        Commands::Update {
            directories,
            check,
            dont_update_version,
        } => update(directories, check, dont_update_version)
            .await
            .unwrap_or_else(|err| {
                eprintln!(
                    "{} {}",
                    "Error updating server and/or plugins! Error:".red(),
                    err
                )
            }),
        Commands::Add(add) => match add.to_add {
            AddCommands::Plugin {
                directory,
                name,
                url,
                overwrite,
                here,
            } => {
                add_plugin_to_existing_server(directory, name, url, overwrite, here)
                    .await
                    .unwrap_or_else(|err| {
                        eprintln!("{} {}", "Error adding plugin! Error:".red(), err)
                    });
            }
            AddCommands::Paper {
                directory,
                accept_eula,
                aikars_flags,
                dont_generate_start_scripts,
                overwrite,
                jar_only,
                version,
            } => {
                println!(
                    "{} {} {} '{}'",
                    "Adding".green(),
                    "Paper".bold().cyan(),
                    "to directory:".green(),
                    directory
                );
                println!("---");
                generate_server(
                    &directory,
                    false,
                    aikars_flags,
                    accept_eula,
                    dont_generate_start_scripts,
                    false,
                    overwrite,
                    jar_only,
                    false,
                    version,
                    &config,
                )
                .await;
            }
            AddCommands::BungeeCord {
                directory,
                aikars_flags,
                dont_generate_start_scripts,
                overwrite,
                jar_only,
                here,
            } => {
                println!(
                    "{} {} {} '{}'",
                    "Adding".green(),
                    "BungeeCord".bold().cyan(),
                    "to directory:".green(),
                    directory
                );
                println!("---");
                generate_server(
                    &directory,
                    true,
                    aikars_flags,
                    false,
                    dont_generate_start_scripts,
                    true,
                    overwrite,
                    jar_only,
                    here,
                    None,
                    &config,
                )
                .await;
            }
            AddCommands::StartFile {
                directory,
                aikars_flags,
                overwrite,
            } => {
                generate_start_script_paper(&directory, aikars_flags, false, overwrite)
                    .unwrap_or_else(|err| {
                        eprintln!("{} {}", "Error adding start script! Error:".red(), err);
                    });
            }
        },
        Commands::Config { regenerate } => {
            config.open_config(regenerate).unwrap_or_else(|err| {
                eprintln!("{} {}", "Error opening config! Error:".red(), err)
            });
        }
    }
}
