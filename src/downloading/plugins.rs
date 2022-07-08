use std::{collections::HashMap, path::Path};

use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PluginsBuildData {
    artifacts: Vec<Artifact>,
    full_display_name: String,
    number: u64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Artifact {
    file_name: String,
    relative_path: String,
}

pub async fn download_plugins(
    path: &Path,
    plugins: &Vec<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let plugins_path_folder = path.join("plugins");
    let plugins_folder = plugins_path_folder.as_path();

    create_dir_all(plugins_folder).await?;

    for plugin in plugins {
        for (name, url) in plugin.iter() {
            let client = Client::builder().build()?;
            let res = client.get(format!("{}/lastStableBuild/api/json", url)).send().await?;
            let json_data = res.json::<PluginsBuildData>().await?;

            let mut jar_file = File::create(
                plugins_folder
                    .clone()
                    .join(&json_data.artifacts[0].file_name),
            )
            .await?;

            println!(
                "   Downloading plugin {} build {}",
                name.bold().yellow(),
                json_data.number.to_string().bold().yellow()
            );

            let mut jar_stream = client
                .get(format!(
                    "{}artifact/{}",
                    json_data.url, json_data.artifacts[0].relative_path
                ))
                .send()
                .await?
                .bytes_stream();

            // Setting the length to 1200 by default for now until I can figure out how
            // to get the length of the byte stream (jar_stream) without having it be consumed or have it's ownership taken.
            let bar = ProgressBar::new(10);
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("   [{bytes_per_sec}] {bar:50.green/blue} {pos:>6}/?")
                    .progress_chars("█▒-"),
            );

            while let Some(item) = jar_stream.next().await {
                bar.inc(1);
                jar_file.write_all(&item.unwrap()).await?;
            }
            bar.finish_at_current_pos();
        }
    }
    Ok(())
}
