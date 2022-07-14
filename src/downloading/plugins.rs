use std::{collections::HashMap, path::Path};

use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{downloading::BuildData, gen::version_file::add_plugin_to_version_file};

pub async fn download_plugin(
    path: &Path,
    plugin: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let plugins_path_folder = path.join("plugins");
    let plugins_folder = plugins_path_folder.as_path();

    create_dir_all(plugins_folder).await?;

    for (name, url) in plugin.iter() {
        let client = Client::builder().build()?;
        let res = client
            .get(format!("{}/lastStableBuild/api/json", url))
            .send()
            .await?;
        let json_data = res.json::<BuildData>().await?;

        let mut jar_file = File::create(
            &(*plugins_folder).join(&json_data.artifacts.as_ref().unwrap()[0].file_name),
        )
        .await?;

        println!(
            "   Downloading plugin {} build {}",
            name.bold().yellow(),
            json_data
                .build
                .as_ref()
                .unwrap()
                .to_string()
                .bold()
                .yellow()
        );

        let mut jar_stream = client
            .get(format!(
                "{}artifact/{}",
                json_data.url.as_ref().unwrap(),
                json_data.artifacts.as_ref().unwrap()[0].relative_path
            ))
            .send()
            .await?
            .bytes_stream();

        // Setting the length to 1200 by default for now until I can figure out how
        // to get the length of the byte stream (jar_stream) without having it be consumed or have it's ownership taken.
        let bar = ProgressBar::new(10);
        bar.enable_steady_tick(100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("    [{bytes_per_sec}] {bar:50.green/blue} {spinner}")
                .progress_chars("█▒-")
                .tick_strings(&["◜", "◠", "◝", "◞", "◡", "◟"]),
        );

        while let Some(item) = jar_stream.next().await {
            bar.inc(1);
            jar_file.write_all(&item.unwrap()).await?;
        }
        bar.finish_at_current_pos();

        add_plugin_to_version_file(
            plugins_folder,
            name,
            json_data.build.unwrap(),
            json_data.artifacts.as_ref().unwrap()[0].file_name.clone(),
            url,
        )
        .await?;
    }

    Ok(())
}

pub async fn download_plugins(
    path: &Path,
    plugins: &Vec<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for plugin in plugins {
        download_plugin(path, plugin).await?;
    }
    Ok(())
}
