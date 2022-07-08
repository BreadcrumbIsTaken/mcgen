use clap::{/*ArgEnum, */Parser, Subcommand};

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
    // /// Updates plugins or server/bungeecord versions.
    // Update {
    //     /// Update the plugins or the server versions.
    //     #[clap(value_parser, arg_enum, default_value = "both")]
    //     update: ToUpdate,
    // },
    /// Opens the config in your computer's default text editor.
    Config {},
    // /// Add a Paper server or BungeeCord network to an already existing server.
    // Add {
    //     /// Add a Paper server or BungeeCord network to an already existing server.
    //     #[clap(value_parser, arg_enum)]
    //     add: ToAdd,

    //     #[clap(long, short, value_parser)]
    //     /// Choose whether or not to create a new directory to store contents of download in.
    //     create_dir: bool,
    // },
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
// pub enum ToUpdate {
//     /// Update plugins.
//     Plugins,
//     /// Update the server.
//     Server,
//     /// Update the server and plugins. (Default)
//     Both,
// }

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
// #[clap(rename_all = "lowercase")]
// pub enum ToAdd {
//     Paper,
//     BungeeCord,
// }
