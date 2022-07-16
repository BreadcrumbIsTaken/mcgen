use crate::{
    config::Config,
    downloading::{bungeecord::download_bungeecord, paper::download_paper},
};

use super::start_script::{generate_start_script_bungeecord, generate_start_script_paper};

use colored::*;

/// Generates a new server.
///
/// # Panics
///
/// Will panic:
/// - if Paper or BungeeCord fails to download,
/// - if the start script for Paper or BungeeCord fails to be generated
pub async fn generate_server(
    dir: &str,
    using_bungeecord: bool,
    aikars_flags: bool,
    accept_eula: bool,
    dont_generate_start_scripts: bool,
    only_bungeecord: bool,
    overwrite: bool,
    jar_only: bool,
    config: &Config<'_>,
) {
    if using_bungeecord {
        download_bungeecord(dir, overwrite, jar_only, Some(config))
            .await
            .unwrap_or_else(|err| {
                eprintln!("{} {}", "Error downloading BungeeCord!".red(), err);
                std::process::exit(1);
            });

        if !dont_generate_start_scripts || !jar_only {
            generate_start_script_bungeecord(dir, overwrite).unwrap_or_else(|err| {
                eprintln!(
                    "{} {}",
                    "Error generating BungeeCord start script!".red(),
                    err
                );
                std::process::exit(1);
            });
        }
    }

    if !only_bungeecord {
        download_paper(
            dir,
            using_bungeecord,
            accept_eula,
            overwrite,
            jar_only,
            Some(config),
        )
        .await
        .unwrap_or_else(|err| {
            eprintln!("{} {}", "Error downloading Paper!".red(), err);
            std::process::exit(1);
        });

        if !dont_generate_start_scripts || !jar_only {
            generate_start_script_paper(dir, aikars_flags, using_bungeecord, overwrite)
                .unwrap_or_else(|err| {
                    eprintln!("{} {}", "Error generating Paper start script!".red(), err);
                    std::process::exit(1);
                });
        }
    }
}
