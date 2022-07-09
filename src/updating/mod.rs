pub mod versions;

// use std::path::Path;

use std::fmt::Write as _;
use std::{ffi::OsStr, path::Path};

use colored::*;
use walkdir::WalkDir;

use crate::updating::versions::{
    check_if_plugin_behind, check_if_server_behind, read_version_file, VersionData,
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

    let mut version_data: Vec<VersionData> = Vec::new();

    if directories.is_empty() {
        let file = Path::new("mcgen.txt");
        let data = read_version_file(file);
        version_data.push(data?);
    } else {
        for directory in directories.clone() {
            if Path::new(&directory).exists() {
                for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
                    if entry.path().file_name() == Some(OsStr::new("mcgen.txt")) {
                        let data = read_version_file(entry.path());
                        version_data.push(data?);
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
        if let Some(server_list) = data.server {
            for server in server_list {
                let mut to_print = String::new();
                // (bool, bool, u32)
                // (version is behind, build is behind, build behind by how much)
                // example: (false, true, 7) version is up to date, build is not up to date, build is behind by 7
                let behind = check_if_server_behind(&server).await?;
                if behind.0 {
                    let _ = write!(to_print, "Version is behind! ");
                }
                if behind.1 {
                    let _ = write!(to_print, "Build is behind by {}! ", behind.2);
                }
                for name in server.keys() {
                    if behind.0 || behind.1 {
                        println!("{} is {} up to date. {}", name, "not".underline(), to_print);
                    } else {
                        println!("{} {}", name.bold().cyan(), "is up to date!".underline());
                    }
                }
            }
        }
        if let Some(plugin_list) = data.plugins {
            for plugin in plugin_list {
                let behind = check_if_plugin_behind(&plugin).await?;
                for name in plugin.keys() {
                    if behind.0 {
                        println!(
                            "Plugin '{}' is {} up to date. Build is behind by {}",
                            name.yellow(),
                            "not".underline(),
                            behind.1
                        );
                    } else {
                        println!("{} {}", name.yellow(), "is up to date!".underline());
                    }
                }
            }
        }
    }

    if !check {
        // Download stuff
    }

    Ok(())
}
