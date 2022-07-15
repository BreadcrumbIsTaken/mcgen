use std::{
    collections::HashMap,
    fs::{create_dir_all, File, remove_file},
    io::{Read, Write},
    path::Path,
};

use colored::*;
use edit::edit_file;
use serde::{Deserialize, Serialize};

use crate::consts::config::DEFAULT_CONFIG_STRING;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub default_plugins: Option<DefaultPlugins>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultPlugins {
    pub paper_plugins: Option<Vec<HashMap<String, String>>>,
    pub bungeecord_plugins: Option<Vec<HashMap<String, String>>>,
}

/// Holds data on the config file.
pub struct Config<'a> {
    path: &'a Path,
    exists: bool,
    pub config: Option<ConfigData>,
}

impl<'a> Config<'a> {
    pub fn new(config_path: &'a Path) -> Self {
        Self {
            path: config_path,
            exists: false,
            config: None,
        }
    }

    pub fn init_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.path.exists() || !self.path.join("config.yml").exists() {
            println!(
                "{}",
                "The config does not exist yet! Creating it right now. It will open right after this. . .".yellow()
            );
            create_dir_all(self.path)?;
            if !self.exists {
                self.create_config().expect("Error creating config!");
            }
        }

        self.read_config()?;

        Ok(())
    }

    pub fn open_config(&mut self, regenerate: bool) -> Result<(), Box<dyn std::error::Error>> {
        if regenerate {
            remove_file(self.path.join("config.yml"))?;
            println!("{}", "Config has been regenerated! Opening. . .".green());
            self.create_config()?;
        }
        edit_file(self.path).unwrap_or_else(|err| {
            eprintln!(
                "{} {} {}",
                "Error opening config file! You can open it yourself here:".red(),
                self.path.display(),
                err
            );
            std::process::exit(1);
        });

        self.read_config()?;

        Ok(())
    }

    fn create_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_file = File::create(self.path.join("config.yml"))?;
        config_file.write_all(DEFAULT_CONFIG_STRING.as_bytes())?;
        self.exists = true;
        Ok(())
    }

    fn read_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut contents = String::new();
        File::options()
            .read(true)
            .open(self.path.join("config.yml"))?
            .read_to_string(&mut contents)?;
        let config: ConfigData = serde_yaml::from_str(&contents)?;
        self.config = Some(config);
        Ok(())
    }
}
