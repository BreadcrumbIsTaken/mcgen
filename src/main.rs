mod cli;
mod sources;

use std::path::Path;

use clap::Parser;
use cli::{Cli, Commands};
use sources::registry::{SourceRegistry, Source};

#[tokio::main]
async fn main() {
    // let cli = Cli::parse();

    let mut registry = SourceRegistry::new();
    let test_source = TestSource::new("Test".to_string(), Path::new("test.txt"));
    registry.add_source(test_source);
    registry._run_all();

    // match cli.commands {
    //     Commands::Gen {
    //         dir,
    //         bungeecord,
    //         aikars_flags,
    //         accept_eula,
    //         dont_generate_start_scripts,
    //         version,
    //     } => todo!(),
    //     Commands::Update {
    //         directories,
    //         check,
    //         dont_update_version,
    //     } => todo!(),
    //     Commands::Add(_) => todo!(),
    //     Commands::Config { regenerate } => todo!(),
    // }
}

struct TestSource<'a> {
    label: String,
    config_path: &'a Path
}

impl<'a> Source<'a> for TestSource<'a> {
    fn get_label(&self) -> &String {
        &self.label
    }

    fn set_label(&mut self, label: &String) {
        self.label = label.to_string();
    }

    fn get_config_path(&self) -> &'a Path {
        self.config_path
    }

    fn set_config_path(&mut self, path: &'a Path) {
        self.config_path = path;
    }

    fn download(&self) {
        println!("Download!");
    }

    fn run(&self) {
        println!("Run!");
        self.download();
    }
}

impl<'a> TestSource<'a> {
    pub fn new(label: String, config_path: &'a Path) -> Self {
        Self {
            label,
            config_path
        }
    }
}
