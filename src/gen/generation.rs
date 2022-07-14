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
    config: &Config<'_>,
) {
    if using_bungeecord {
        download_bungeecord(dir, Some(config))
            .await
            .unwrap_or_else(|err| {
                eprintln!("{} {}", "Error downloading BungeeCord!".red(), err);
                std::process::exit(1);
            });

        generate_start_script_bungeecord(dir).unwrap_or_else(|err| {
            eprintln!(
                "{} {}",
                "Error generating BungeeCord start script!".red(),
                err
            );
            std::process::exit(1);
        });
    }

    download_paper(dir, using_bungeecord, Some(config))
        .await
        .unwrap_or_else(|err| {
            eprintln!("{} {}", "Error downloading Paper!".red(), err);
            std::process::exit(1);
        });

    generate_start_script_paper(dir, aikars_flags, using_bungeecord).unwrap_or_else(|err| {
        eprintln!("{} {}", "Error generating Paper start script!".red(), err);
        std::process::exit(1);
    });
}
