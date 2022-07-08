use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

use crate::consts::scripts::{
    AIKARS_FLAGS_PAPER_START_SCRIPT, BUNGEECORD_START_SCRIPT, REGULAR_PAPER_START_SCRIPT,
};

use colored::*;

/// Generates a basic start script for BungeeCord.
///
/// The script's file extension will be different based on which operating system you use.
/// The RAM given is defaulted to 1 gigabyte. Feel free to change this as you please.
pub fn generate_start_script_bungeecord(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&dir).join("bungeecord");

    #[cfg(all(target_os = "linux", target_os = "macos"))]
    let file_path = path.join("start.sh");
    #[cfg(target_os = "windows")]
    let file_path = path.join("start.bat");

    if file_path.exists() {
        Err(Box::new(Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "The directory, '{}', already exists!",
                path.display().to_string().cyan()
            ),
        )))
    } else {
        let mut start_script = File::create(file_path)?;

        start_script.write_all(BUNGEECORD_START_SCRIPT.as_bytes())?;

        // println!(
        //     "{}",
        //     "Successfully generated BungeeCord start script!".green()
        // );
        Ok(())
    }
}

/// Generates a basic start script for Paper.
///
/// The script's file extension will be different based on which operating system you use.
/// The RAM given is defaulted to 1 gigabyte. Feel free to change this as you please.
pub fn generate_start_script_paper(
    dir: &str,
    aikars_flags: bool,
    using_bungeecord: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    let path = if using_bungeecord {
        path.join("server")
    } else {
        path.to_path_buf()
    };

    #[cfg(all(target_os = "linux", target_os = "macos"))]
    let file_path = path.join("start.sh");
    #[cfg(target_os = "windows")]
    let file_path = path.join("start.bat");

    if file_path.exists() {
        Err(Box::new(Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "The directory, '{}', already exists!",
                path.display().to_string().cyan()
            ),
        )))
    } else {
        let mut start_script = File::create(file_path)?;
        if aikars_flags {
            start_script.write_all(AIKARS_FLAGS_PAPER_START_SCRIPT.as_bytes())?;
        } else {
            start_script.write_all(REGULAR_PAPER_START_SCRIPT.as_bytes())?;
        }

        // println!("{}", "Successfully generated Paper start script!".green());

        Ok(())
    }
}
