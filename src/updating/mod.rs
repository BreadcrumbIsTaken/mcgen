pub mod plugin;
pub mod server;
pub mod versions;

use std::{collections::HashMap, ffi::OsStr, path::Path};

use colored::*;
use walkdir::WalkDir;

use crate::updating::{
    plugin::update_plugin,
    server::update_server,
    versions::{read_version_file, VersionData},
};

pub async fn update(
    directories: Vec<String>,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let dirs: String = if directories.is_empty() {
        "Current directory".to_string()
    } else {
        directories.clone().join(", ")
    };

    println!("{} {}", "Updating directories:".green(), dirs,);
    println!("---");

    let mut version_data: Vec<HashMap<String, VersionData>> = Vec::new();

    if directories.is_empty() {
        let file = Path::new("mcgen.txt");
        let data = read_version_file(file)?;
        version_data.push(HashMap::from([(file.display().to_string(), data)]));
    } else {
        for directory in directories.clone() {
            if Path::new(&directory).exists() {
                for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
                    if entry.path().file_name() == Some(OsStr::new("mcgen.txt")) {
                        let data = read_version_file(entry.path())?;
                        version_data
                            .push(HashMap::from([(entry.path().display().to_string(), data)]));
                    }
                }
            } else {
                eprintln!(
                    "{} '{}'{}",
                    "The directory,".red(),
                    directory,
                    ", does not exist!".red()
                );
            }
        }
    }

    for data in version_data {
        for (path, version_data) in data {
            if let Some(server_list) = version_data.server {
                for server in server_list {
                    update_server(&server, Path::new(&path), check).await?;
                }
            }
            if let Some(plugin_list) = version_data.plugins {
                for plugin in plugin_list {
                    update_plugin(&plugin, Path::new(&path), check).await?;
                }
            }
        }
    }

    Ok(())
}
