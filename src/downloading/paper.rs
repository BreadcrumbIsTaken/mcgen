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
    consts::urls::PAPER_JSON_API_URL,
    downloading::{plugins::download_plugins, BuildData},
    gen::{eula::generate_eula, version_file::generate_version_file},
};

/// Downloads the latest Paper version.
pub async fn download_paper(
    dir: &str,
    using_bungeecord: bool,
    accept_eula: bool,
    overwrite: bool,
    config: Option<&Config<'_>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    let paper_path = if using_bungeecord {
        path.join("server")
    } else {
        path.to_path_buf()
    };

    if paper_path.join("paper.jar").exists() && !overwrite {
        eprintln!(
            "{} '{}' {}",
            "The directory,".red(),
            path.join("paper.jar").display(),
            "already exists!".red()
        );
        std::process::exit(1);
    } else {
        // Expect the `Err` with a permissions error.
        // The path does not exist so it cannot error due to the path already existing.
        create_dir_all(paper_path.clone())
            .await
            .expect("Could not create the paper directory! Do you have the correct permissions?");

        let client = Client::builder().build()?;
        let res = client.get(PAPER_JSON_API_URL).send().await?;

        let json_data = res.json::<BuildData>().await?;

        let latest_version = json_data
            .versions
            .as_ref()
            .unwrap()
            .last()
            .ok_or("Could not get latest Paper version.");

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
        bar.enable_steady_tick(100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bytes_per_sec}] {bar:50.green/blue} {spinner}")
                .progress_chars("█▒-")
                .tick_strings(&["◜", "◠", "◝", "◞", "◡", "◟"]),
        );

        while let Some(item) = jar_data.next().await {
            jar_file.write_all(&item.unwrap()).await?;
            bar.inc(1);
        }
        bar.finish_at_current_pos();

        if let Some(conf) = &config {
            if let Some(data) = &conf.config {
                let plugins = data
                    .default_plugins
                    .as_ref()
                    .unwrap()
                    .paper_plugins
                    .as_ref();
                if let Some(plugins_list) = plugins {
                    download_plugins(paper_path.clone().as_path(), plugins_list, overwrite).await?;
                }
            }
        }

        generate_version_file(
            paper_path.clone().as_path(),
            format!(
                r#"server:
    - Paper: 
        version: {}
        build: {}"#,
                latest_version.unwrap(),
                latest_build
            ),
        )?;

        if accept_eula {
            generate_eula(&paper_path.display().to_string()).unwrap_or_else(|err| {
                eprintln!("{} {}", "Error generating EULA! Error:".red(), err);
                std::process::exit(1);
            });
        }
    }

    Ok(())
}
