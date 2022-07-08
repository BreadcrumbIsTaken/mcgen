use crate::{
    config::create::Config,
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
        download_bungeecord(dir, &config)
            .await
            .expect(&format!("{}", "Error downloading BungeeCord!".red()));

        generate_start_script_bungeecord(dir).expect(&format!(
            "{}",
            "Error generating BungeeCord start script!".red()
        ));
    }

    download_paper(dir, using_bungeecord, &config)
        .await
        .expect(&format!("{}", "Error downloading Paper!".red()));

    generate_start_script_paper(dir, aikars_flags, using_bungeecord)
        .expect(&format!("{}", "Error generating Paper start script!".red()));
}
