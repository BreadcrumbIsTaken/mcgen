//! # mcgen
//! ## Generate a Minecraft server in seconds!
mod cli;
mod config;
mod consts;
mod downloading;
mod gen;

use clap::Parser;
use cli::{Cli, Commands/* , ToUpdate*/};
use colored::*;
use dirs::config_dir;
use std::path::Path;

use crate::{config::create::Config, gen::generation::generate_server};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config_path = format!("{}\\mcgen", config_dir().unwrap().display());
    let mut config = Config::new(Path::new(&config_path));
    config.init_config().expect("Error initiating config!");

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
        // Commands::Update { update } => match update {
        //     ToUpdate::Plugins => {
        //         println!("Updating plugins");
        //     }
        //     ToUpdate::Server => {
        //         println!("Updating server");
        //     }
        //     ToUpdate::Both => {
        //         println!("Updating both");
        //     }
        // },
        Commands::Config {} => {
            config.open_config().expect("Error opening config!");
        }
        // Commands::Add {
        //     add: _,
        //     create_dir: _,
        // } => {}
    }
}
