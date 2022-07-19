use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::updating::versions::{PluginVersionDetails, VersionData};

pub fn generate_version_file(
    path: &Path,
    contents: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let versions_file = File::create(path.join("mcgen.txt"));
    versions_file?.write_all(contents.as_bytes())?;

    Ok(())
}

pub async fn add_plugin_to_version_file(
    path: &Path,
    name: &String,
    build: u32,
    file_name: String,
    url: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = path.join("mcgen.txt");

    let mut versions_file = OpenOptions::new().write(true).create(true).open(&file)?;

    versions_file.write_all(b"plugins:")?;

    let mut previous_data: VersionData = serde_yaml::from_str(&fs::read_to_string(&file)?)?;
    let mut new_contents = previous_data.clone();

    match previous_data.plugins {
        Some(ref mut previous) => {
            for i in 0..previous.len() {
                // Check if the plugin's name is already in the previous contents.
                if previous.get(i).unwrap().get(name).is_some() {
                    // Get list of plugins to add.
                    if let Some(ref mut content) = new_contents.plugins {
                        // Get the map of data from the new plugin.
                        if let Some(hm) = content.get(i).cloned() {
                            // Get the plugin's data
                            if let Some(mut data) = hm.get(name).cloned() {
                                let position = content.iter().position(|x| x == &hm);
                                if let Some(pos) = position {
                                    content.remove(pos);
                                }
                                data.build = build;
                                data.file_name = Some(file_name.clone());
                                data.url = url.to_string();
                                let new_plugin = HashMap::from([(name.to_owned(), data)]);
                                if !content.contains(&new_plugin) {
                                    // Add plugin to list of new data to be written.
                                    content.push(new_plugin);
                                }
                            }
                        }
                    }
                // Not in mcgen file; plugin is being added.
                } else {
                    let data = PluginVersionDetails {
                        build,
                        file_name: Some(file_name.clone()),
                        url: url.to_string(),
                    };
                    match new_contents.plugins {
                        Some(ref mut plugins) => {
                            let plugin = HashMap::from([(name.to_owned(), data)]);
                            let position = plugins.iter().position(|x| x == &plugin);
                            if let Some(pos) = position {
                                plugins.remove(pos);
                            }
                            if !plugins.contains(&plugin) {
                                plugins.push(plugin);
                            }
                        }
                        None => {
                            new_contents.plugins =
                                Some(vec![HashMap::from([(name.to_owned(), data)])])
                        }
                    }
                }
            }
        }
        None => {
            let data = PluginVersionDetails {
                build,
                file_name: Some(file_name),
                url: url.to_string(),
            };
            new_contents.plugins = Some(vec![HashMap::from([(name.to_owned(), data)])]);
        }
    }

    let to_write = serde_yaml::to_string(&new_contents.plugins)
        .unwrap()
        .replace("---", "");
    versions_file.write_all(to_write.as_bytes())?;

    Ok(())
}
