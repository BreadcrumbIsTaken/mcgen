use std::path::Path;

use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{
    config::Config,
    consts::urls::BUNGEECORD_JSON_API_URL,
    downloading::{plugins::download_plugins, BuildData},
    gen::version_file::generate_version_file,
};

/// Downloads the latest BungeeCord jar to a given path.
pub async fn download_bungeecord(
    dir: &str,
    overwrite: bool,
    jar_only: bool,
    here: bool,
    config: Option<&Config<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);

    let bungeecord_path = if here {
        path.to_path_buf()
    } else {
        path.join("bungeecord")
    };
    let bungeecord_path = bungeecord_path.as_path();

    if bungeecord_path.join("BungeeCord.jar").exists() && !overwrite {
        eprintln!(
            "{} '{}'!",
            "BungeeCord already exists in directory,".red(),
            path.display()
        );
        std::process::exit(1);
    } else {
        create_dir_all(bungeecord_path).await?;

        let client = Client::builder().build()?;
        let res = client.get(BUNGEECORD_JSON_API_URL).send().await?;

        let json_data = res.json::<BuildData>().await?;

        let mut jar_file =
            File::create(bungeecord_path.join(&json_data.artifacts.as_ref().unwrap()[0].file_name))
                .await?;

        println!(
            "Downloading {} build {}",
            "BungeeCord".bold().cyan(),
            json_data.build.as_ref().unwrap().to_string().bold().cyan(),
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
        let bar = ProgressBar::new(1200);
        bar.enable_steady_tick(100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bytes_per_sec}] {bar:50.green/blue} {spinner}")
                .progress_chars("█▒-")
                .tick_strings(&["◜", "◠", "◝", "◞", "◡", "◟"]),
        );

        while let Some(item) = jar_stream.next().await {
            bar.inc(1);
            jar_file.write_all(&item.unwrap()).await?;
        }
        bar.finish_at_current_pos();

        generate_version_file(
            bungeecord_path,
            format!(
                r#"server:
    - BungeeCord:
        build: {}"#,
                json_data.build.as_ref().unwrap()
            ),
        )?;

        if !jar_only {
            if let Some(conf) = &config {
                if let Some(data) = &conf.config {
                    let plugins = data
                        .default_plugins
                        .as_ref()
                        .unwrap()
                        .bungeecord_plugins
                        .as_ref();
                    if let Some(plugins_list) = plugins {
                        download_plugins(bungeecord_path, plugins_list, overwrite).await?;
                    }
                }
            }
        }
    }
    Ok(())
}
