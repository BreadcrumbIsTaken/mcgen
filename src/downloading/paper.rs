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
    config::create::Config, consts::urls::PAPER_JSON_API_URL,
    downloading::plugins::download_plugins,
};

/// Stores data on the latest Minecraft versions for Paper.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MinecraftVersionsForPaperData {
    versions: Vec<String>,
}

/// Stores data on the latest Paper builds.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PaperVersionBuildsData {
    builds: Vec<u64>,
}

/// Downloads the latest Paper version.
pub async fn download_paper(
    dir: &str,
    using_bungeecord: bool,
    config: &Config<'_>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    let paper_path = if using_bungeecord {
        path.join("server")
    } else {
        path.to_path_buf()
    };

    if paper_path.exists() {
        Err(Box::new(Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "The directory, '{}', already exists!",
                path.display().to_string()
            ),
        )))
    } else {
        // Expect the `Err` with a permissions error.
        // The path does not exist so it cannot error due to the path already existing.
        create_dir_all(paper_path.clone())
            .await
            .expect("Could not create the paper directory! Do you have the correct permissions?");

        let client = Client::builder().build()?;
        let res = client.get(PAPER_JSON_API_URL).send().await?;

        let json_data = res.json::<MinecraftVersionsForPaperData>().await?;

        let latest_version = json_data
            .versions
            .last()
            .ok_or("Could not get latest Paper version.");

        let verion_builds = client
            .get(format!(
                "{}/versions/{}",
                PAPER_JSON_API_URL,
                latest_version.unwrap()
            ))
            .send()
            .await?;
        let version_builds_json_data = verion_builds.json::<PaperVersionBuildsData>().await?;

        let latest_build = *version_builds_json_data
            .builds
            .iter()
            // Get the latest version number (highest number)
            .max_by(|x, y| x.cmp(y))
            .unwrap();

        let file_name = format!("paper-{}-{}.jar", latest_version.unwrap(), latest_build);

        let mut jar_file = File::create(paper_path.join("paper.jar")).await?;

        let paper_download_url = format!(
            "{}/versions/{}/builds/{}/downloads/{}",
            PAPER_JSON_API_URL,
            latest_version.unwrap(),
            latest_build,
            file_name
        );

        println!(
            "Downloading {} version {}, build {}",
            "Paper".bold().cyan(),
            latest_version.unwrap().bold().cyan(),
            latest_build.to_string().bold().cyan(),
        );

        let mut jar_data = client.get(paper_download_url).send().await?.bytes_stream();

        let bar = ProgressBar::new(2100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bytes_per_sec}] {bar:50.green/blue} {pos:>6}/?")
                .progress_chars("█▒-"),
        );

        while let Some(item) = jar_data.next().await {
            jar_file.write_all(&item.unwrap()).await?;
            bar.inc(1);
        }
        bar.finish_at_current_pos();

        if let Some(data) = &config.config {
            let plugins = data
                .default_plugins
                .as_ref()
                .unwrap()
                .paper_plugins
                .as_ref();
            if let Some(plugins_list) = plugins {
                download_plugins(paper_path.clone().as_path(), plugins_list).await?;
            }
        }

        //     let versions_file = File::create(paper_path.join("mcgen.yml")).await;
        //     let versions_file_contents = format!(
        //         r#"servers:
        // - Paper:
        //     version: {}
        //     build: {}"#,
        //         latest_version.unwrap(),
        //         latest_build
        //     );
        //     versions_file
        //         .unwrap()
        //         .write_all(versions_file_contents.as_bytes())
        //         .await
        //         .unwrap();

        Ok(())
    }
}
