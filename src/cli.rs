use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "Breadcrumb", version = "0.1.0", about = "Generate a barebones Minecraft server in seconds!", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Generate a new server.
    Gen {
        /// The name of the directory to place contents of the Minecraft server
        #[clap(value_parser)]
        dir: String,

        /// Choose whether or not to have a BungeeCord network.
        #[clap(long, short, value_parser)]
        bungeecord: bool,

        /// Choose whether or not to use Aikar's Flags (reference: <https://aikar.co/mcflags.html>)
        #[clap(long, short, value_parser)]
        aikars_flags: bool,
    },
    /// Updates plugins or server/bungeecord versions.
    #[clap(
        long_about = "Updates plugins or server/bungeecord versions. mcgen keeps track of versions via a text file called 'mcgen.txt'. mcgen will look for the mcgen.txt file in the current directory. If you want to update plugins in more than one directory (most likely for multiple servers or use of BungeeCord), then you need to pass in the directories to update. Is recursive if directories are given."
    )]
    Update {
        /// A comma-then-space seperated list of directories to update the plugins/servers. Will look for the mcgen.txt file in the current directory if not set.
        #[clap(value_parser)]
        directories: Vec<String>,

        /// Will check for any updates, but will not install them.
        #[clap(long, short, value_parser)]
        check: bool,
    },
    /// Opens the config in your computer's default text editor.
    Config {},
}
