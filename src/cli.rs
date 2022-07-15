use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[clap(name = "mcgen", author = "Breadcrumb", version = "0.1.0", about = "Generate a barebones Minecraft server in seconds!", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    /// Generate a new server.
    Gen {
        /// The name of the directory to place contents of the Minecraft server
        #[clap(value_parser)]
        dir: String,

        /// Choose whether or not to have a BungeeCord network.
        #[clap(long, short, value_parser)]
        bungeecord: bool,

        /// Choose whether or not to use Aikar's Flags (https://aikar.co/mcflags.html)
        #[clap(long = "aikars-flags", short, value_parser)]
        aikars_flags: bool,

        /// Accept to Minecraft's EULA. Will create the file automatically with `eula` set to `true`. By using this option, you agree to accept the EULA: https://aka.ms/MinecraftEULA
        #[clap(
            long,
            short = 'e',
            value_parser,
            verbatim_doc_comment
        )]
        accept_eula: bool,

        /// By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.
        #[clap(long = "no-start-scripts", short = 'n', value_parser, verbatim_doc_comment)]
        dont_generate_start_scripts: bool,
    },
    /// Updates plugins or server/bungeecord versions.
    #[clap(
        long_about = "Updates plugins or server/bungeecord versions. mcgen keeps track of versions via a text file called 'mcgen.txt'. mcgen will look for the mcgen.txt file in the current directory. If you want to update plugins in more than one directory (most likely for multiple servers or use of BungeeCord), then you need to pass in the directories to update. Is recursive if directories are given.",
        arg_required_else_help = true
    )]
    Update {
        /// A comma-then-space seperated list of directories to update the plugins/servers. Will look for the mcgen.txt file in the current directory if not set.
        #[clap(value_parser)]
        directories: Vec<String>,

        /// Will check for any updates, but will not install them.
        #[clap(long, short, value_parser)]
        check: bool,
    },
    /// Add additional items.
    Add(Add),
    /// Opens the config in your computer's default text editor.
    Config {
        /// Will delete the contents of the current config and create a new one with the default values and then open it.
        #[clap(long, short, value_parser)]
        regenerate: bool,
    },
}

#[derive(Args, Debug, Clone)]
pub struct Add {
    #[clap(subcommand)]
    pub to_add: AddCommands,
}

#[derive(Subcommand, Debug, Clone)]
#[clap(rename_all = "lower")]
pub enum AddCommands {
    /// Add a plugin. Must have a Jenkins API
    Plugin {
        /// The directory to add the plugin to.
        #[clap(value_parser)]
        directory: String,

        /// Name of the plugin.
        #[clap(value_parser)]
        name: String,

        /// URL to download plugin. MUST HAVE A JENKINS API!
        #[clap(value_parser)]
        url: String,
    },
    /// Add a Paper server.
    Paper {
        /// The directory to add Paper to.
        #[clap(value_parser)]
        directory: String,

        /// Accept to Minecraft's EULA. Will create the file automatically with `eula` set to `true`. By using this option, you agree to accept the EULA: https://aka.ms/MinecraftEULA
        #[clap(
            long,
            short = 'e',
            value_parser,
            verbatim_doc_comment
        )]
        accept_eula: bool,

        /// Choose whether or not to use Aikar's Flags (https://aikar.co/mcflags.html)
        #[clap(long = "aikars-flags", short, value_parser)]
        aikars_flags: bool,

        /// By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.
        #[clap(long = "no-start-scripts", short = 'n', value_parser, verbatim_doc_comment)]
        dont_generate_start_scripts: bool,
    },
    /// Add a BungeeCord server.
    BungeeCord {
        /// The directory to add BungeeCord to.
        #[clap(value_parser)]
        directory: String,

        /// Choose whether or not to use Aikar's Flags (https://aikar.co/mcflags.html)
        #[clap(long = "aikars-flags", short, value_parser)]
        aikars_flags: bool,

        /// By using this option mcgen will not generate the start scripts. By not using this flag mcgen continues its default behavior and will generate the start scripts.
        #[clap(long = "no-start-scripts", short = 'n', value_parser, verbatim_doc_comment)]
        dont_generate_start_scripts: bool,
    },
    /// Add a start file.
    StartFile {
        /// The directory to add the start files to.
        #[clap(value_parser)]
        directory: String,
        /// Choose whether or not to use Aikar's Flags (https://aikar.co/mcflags.html)
        #[clap(long = "aikars-flags", short, value_parser)]
        aikars_flags: bool,
    },
}
