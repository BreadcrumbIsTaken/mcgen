use std::{io::Error, path::Path};

use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{
    config::create::Config, consts::urls::BUNGEECORD_JSON_API_URL,
    downloading::plugins::download_plugins,
};

/// Stores BungeeCord build data.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BungeecordBuildData {
    artifacts: Vec<Artifact>,
    full_display_name: String,
    number: u64,
    url: String,
}

/// Stores BungeeCord build artifact data.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Artifact {
    file_name: String,
    relative_path: String,
}

/// Downloads the latest BungeeCord jar to a given path.
pub async fn download_bungeecord(
    dir: &str,
    config: &Config<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    // Note: consider adding error messages.
    if path.exists() {
        Err(Box::new(Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "The directory, '{}', already exists!",
                path.display().to_string()
            ),
        )))
    } else {
        let bungeecord_path = path.join("bungeecord");
        // Expect the `Err` with a permissions error.
        // The path does not exist so it cannot error due to the path already existing.
        create_dir_all(bungeecord_path.clone())
            .await
            .expect("Could not create the server directory! Do you have the correct permissions?");

        let client = Client::builder().build()?;
        let res = client.get(BUNGEECORD_JSON_API_URL).send().await?;

        let json_data = res.json::<BungeecordBuildData>().await?;

        let mut jar_file = File::create(
            bungeecord_path
                .as_path()
                .join(&json_data.artifacts[0].file_name),
        )
        .await?;

        println!(
            "Downloading {} build {}",
            "BungeeCord".bold().cyan(),
            json_data.number.to_string().bold().cyan(),
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
        let bar = ProgressBar::new(1200);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bytes_per_sec}] {bar:50.green/blue} {pos:>6}/?")
                .progress_chars("█▒-"),
        );

        while let Some(item) = jar_stream.next().await {
            bar.inc(1);
            jar_file.write_all(&item.unwrap()).await?;
        }
        bar.finish_at_current_pos();

        if let Some(data) = &config.config {
            let plugins = data
                .default_plugins
                .as_ref()
                .unwrap()
                .bungeecord_plugins
                .as_ref()
                .unwrap();
            download_plugins(bungeecord_path.clone().as_path(), plugins).await?;
        }

        // println!("{:?}", config.config);

        //     let versions_file = File::create(bungeecord_path.join("mcgen.yml")).await;
        //     let versions_file_contents = format!(
        //         r#"servers:
        // - BungeeCord:
        //     build: {}"#,
        //         json_data.number
        //     );
        //     versions_file
        //         .unwrap()
        //         .write_all(versions_file_contents.as_bytes())
        //         .await
        //         .unwrap();

        Ok(())
    }
}
