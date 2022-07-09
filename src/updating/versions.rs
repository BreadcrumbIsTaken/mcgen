use std::{collections::HashMap, fs::File, io::Read, path::Path};

use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    consts::urls::{BUNGEECORD_JSON_API_URL, PAPER_JSON_API_URL},
    downloading::BuildData,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionData {
    pub server: Option<Vec<HashMap<String, ServerVersionDetails>>>,
    pub plugins: Option<Vec<HashMap<String, PluginVersionDetails>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerVersionDetails {
    pub build: u32,
    // This is a String and not a f64 because there could be more than one decimal place in Minecraft versions, such as 1.18.2
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginVersionDetails {
    pub build: u32,
    pub file_name: String,
    pub url: String,
}

pub fn read_version_file(path: &Path) -> Result<VersionData, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    File::options()
        .read(true)
        .open(path)?
        .read_to_string(&mut contents)?;

    let version_data: VersionData = serde_yaml::from_str(&contents)?;

    Ok(version_data)
}

// (bool, bool, u32)
// (version is behind, build is behind, build behind by how much)
// example: (false, true, 7) version is up to date, build is not up to date, build is behind by 7
pub async fn check_if_server_behind(
    server: &HashMap<String, ServerVersionDetails>,
) -> Result<(bool, bool, u32), Box<dyn std::error::Error>> {
    let mut final_values: Result<(bool, bool, u32), Box<dyn std::error::Error>> =
        Ok((true, true, 0));

    for (name, mcgen_file_data) in server {
        let url: &str;
        let paper: bool;

        if name == "BungeeCord" {
            url = BUNGEECORD_JSON_API_URL;
            paper = false;
        } else {
            url = PAPER_JSON_API_URL;
            paper = true;
        }

        let client = Client::builder().build()?;
        let res = client.get(url).send().await?;
        let receiving_version_data = res.json::<BuildData>().await?;

        println!("Checking for updates for {}. . .", name.bold().cyan());

        let version_outdated: bool;
        let build_outdated: bool;

        final_values = if !paper {
            // BungeeCord does not use versions like Paper so it will always be false.
            if mcgen_file_data.build != receiving_version_data.build.unwrap() {
                Ok((
                    false,
                    true,
                    receiving_version_data.build.unwrap() - mcgen_file_data.build,
                ))
            } else {
                Ok((false, false, 0))
            }
        } else {
            // Paper is more complex as it uses it's own API, unlike BungeeCord which uses Jenkins.
            let latest_version = receiving_version_data
                .versions
                .as_ref()
                .unwrap()
                .last()
                .ok_or("Could not get latest Paper version.");

            // Checks if the version numbers are the same. If they aren't, then Paper is outdated and should be updated.
            if *mcgen_file_data.version.as_ref().unwrap() != *latest_version.unwrap() {
                version_outdated = true;
            } else {
                version_outdated = false;
            }

            let version_builds = client
                .get(format!(
                    "{}/versions/{}",
                    PAPER_JSON_API_URL,
                    latest_version.unwrap()
                ))
                .send()
                .await?;

            let version_builds_json_data = version_builds.json::<BuildData>().await?;

            let latest_build = *version_builds_json_data
                .builds
                .as_ref()
                .unwrap()
                .iter()
                // Get the latest version number (highest number)
                .max_by(|x, y| x.cmp(y))
                .unwrap();

            if mcgen_file_data.build != latest_build {
                build_outdated = true;
            } else {
                build_outdated = false;
            }

            Ok((
                version_outdated,
                build_outdated,
                latest_build - mcgen_file_data.build,
            ))
        };
    }

    final_values
}

pub async fn check_if_plugin_behind(
    plugin: &HashMap<String, PluginVersionDetails>,
) -> Result<(bool, u32), Box<dyn std::error::Error>> {
    let mut final_values: Result<(bool, u32), Box<dyn std::error::Error>> = Ok((true, 0));
    for (name, mcgen_file_data) in plugin {
        let client = Client::builder().build()?;
        let res = client
            .get(&format!("{}/lastStableBuild/api/json", mcgen_file_data.url))
            .send()
            .await?;
        let receiving_version_data = res.json::<BuildData>().await?;

        println!("Checking for updates for {}. . .", name.yellow());

        final_values = if mcgen_file_data.build != receiving_version_data.build.unwrap() {
            Ok((
                true,
                receiving_version_data.build.unwrap() - mcgen_file_data.build,
            ))
        } else {
            Ok((false, 0))
        };
    }

    final_values
}
