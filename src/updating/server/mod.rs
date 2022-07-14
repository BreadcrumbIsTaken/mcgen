use std::fmt::Write as _;
use std::{collections::HashMap, path::Path};

use colored::*;
use reqwest::Client;
use tokio::fs;

use crate::downloading::bungeecord::download_bungeecord;
use crate::downloading::paper::download_paper;
use crate::{
    consts::urls::{BUNGEECORD_JSON_API_URL, PAPER_JSON_API_URL},
    downloading::BuildData,
};

use super::versions::ServerVersionDetails;

pub async fn update_server(
    server: &HashMap<String, ServerVersionDetails>,
    path: &Path,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, mcgen_file_data) in server {
        let url: &str;
        let updating_paper: bool;

        if name.to_lowercase() == "bungeecord" {
            url = BUNGEECORD_JSON_API_URL;
            updating_paper = false;
        } else {
            url = PAPER_JSON_API_URL;
            updating_paper = true;
        }

        let client = Client::builder().build()?;
        let res = client.get(url).send().await?;
        let receiving_version_data = res.json::<BuildData>().await?;

        println!("Checking for updates for {}. . .", name.bold().cyan());

        let version_outdated: bool;
        let build_outdated: bool;
        let builds_behind: u32;

        if !updating_paper {
            // BungeeCord does not use versions like Paper so it will always be false.
            if mcgen_file_data.build != receiving_version_data.build.unwrap() {
                version_outdated = false;
                build_outdated = true;
                builds_behind = receiving_version_data.build.unwrap() - mcgen_file_data.build;
            } else {
                version_outdated = false;
                build_outdated = false;
                builds_behind = 0;
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

            builds_behind = latest_build - mcgen_file_data.build;
        }

        let mut to_print = String::new();
        if version_outdated {
            let _ = write!(to_print, "Version is behind! ");
        }
        if build_outdated {
            let _ = write!(to_print, "Build is behind by {}!", builds_behind);
        }

        if !version_outdated && !build_outdated {
            println!("{} {}", name.bold().cyan(), "is up to date!".underline());
        } else if version_outdated || build_outdated {
            println!(
                "{} is {} up to date. {}",
                name.bold().cyan(),
                "not".underline(),
                to_print
            );

            if !check {
                println!("Updating {}. . .", name.bold().cyan());

                if name == "Paper" {
                    fs::remove_file(path.parent().unwrap().join("paper.jar")).await?;
                    download_paper(&path.parent().unwrap().display().to_string(), false, None)
                        .await?;
                } else {
                    fs::remove_file(path.parent().unwrap().join("BungeeCord.jar")).await?;
                    download_bungeecord(&path.parent().unwrap().display().to_string(), None)
                        .await?;
                }
            }
        }
    }

    Ok(())
}
