use std::{collections::HashMap, path::Path};

use colored::*;
use reqwest::Client;
use tokio::fs;

use crate::downloading::{plugins::download_plugin, BuildData};

use super::versions::PluginVersionDetails;

pub async fn update_plugin(
    plugin: &HashMap<String, PluginVersionDetails>,
    path: &Path,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, mcgen_file_data) in plugin {
        let client = Client::builder().build()?;
        let res = client
            .get(&format!("{}/lastStableBuild/api/json", mcgen_file_data.url))
            .send()
            .await?;
        let receiving_version_data = res.json::<BuildData>().await?;

        println!("Checking for updates for plugin '{}'. . .", name.yellow());

        let build_outdated: bool;
        let builds_behind: u32;
        if mcgen_file_data.build != receiving_version_data.build.unwrap() {
            build_outdated = true;
            builds_behind = receiving_version_data.build.unwrap() - mcgen_file_data.build;
        } else {
            build_outdated = false;
            builds_behind = 0;
        }

        if !build_outdated {
            println!(
                "    Plugin '{}' {}",
                name.yellow(),
                "is up to date!".underline()
            );
        } else {
            println!(
                "    Plugin '{}' is {} up to date. Build is behind by {}!",
                name.yellow(),
                "not".underline(),
                builds_behind
            );

            if !check {
                println!("    Updating plugin {}. . .", name.yellow());
                if let Some(plugin_path) = &mcgen_file_data.file_name {
                    if path.parent().unwrap().join(&plugin_path).exists() {
                        fs::remove_file(path.parent().unwrap().join(&plugin_path)).await?;
                    }
                }

                download_plugin(
                    path.parent().unwrap().parent().unwrap(),
                    &HashMap::from([(name.to_owned(), mcgen_file_data.url.to_owned())]),
                    true,
                    false,
                )
                .await?;
            }
        }
    }

    Ok(())
}
