use colored::*;
use std::{fs::File, io::Write, path::Path};
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use crate::consts::scripts::{
    AIKARS_FLAGS_PAPER_START_SCRIPT, BUNGEECORD_START_SCRIPT, REGULAR_PAPER_START_SCRIPT,
};

/// Generates a basic start script for BungeeCord.
///
/// The script's file extension will be different based on which operating system you use.
/// The RAM given is defaulted to 1 gigabyte. Feel free to change this as you please.
pub fn generate_start_script_bungeecord(
    dir: &str,
    overwrite: bool,
    here: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // let path = Path::new(&dir).join("bungeecord");
    let path = if here {
        Path::new(&dir).to_path_buf()
    } else {
        Path::new(&dir).join("bungeecord")
    };
    let path = path.as_path();

    let file_paths = vec![path.join("start.sh"), path.join("start.bat")];
    let mut exists = false;

    for file_path in file_paths.clone() {
        if file_path.exists() {
            exists = true;
        } else {
            exists = false;
        }
    }

    if exists && !overwrite {
        eprintln!(
            "{} '{}'",
            "Start files already exist in directory:".red(),
            path.display()
        );
        std::process::exit(1);
    } else {
        for file_path in file_paths {
            let mut start_script = File::create(file_path)?;

            start_script.write_all(BUNGEECORD_START_SCRIPT.as_bytes())?;
        }
    }
    Ok(())
}

/// Generates a basic start script for Paper.
///
/// The script's file extension will be different based on which operating system you use.
/// The RAM given is defaulted to 1 gigabyte. Feel free to change this as you please.
pub fn generate_start_script_paper(
    dir: &str,
    aikars_flags: bool,
    using_bungeecord: bool,
    overwrite: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    let path = if using_bungeecord {
        path.join("server")
    } else {
        path.to_path_buf()
    };

    let file_paths = vec![path.join("start.sh"), path.join("start.bat")];
    let mut exists = false;

    for file_path in file_paths.clone() {
        if file_path.exists() {
            exists = true;
        } else {
            exists = false;
        }
    }

    if exists && !overwrite {
        eprintln!(
            "{} '{}'",
            "Start files already exist in directory:".red(),
            path.display()
        );
        std::process::exit(1);
    } else {
        for file_path in file_paths {
            let mut aikars_flags_contents = AIKARS_FLAGS_PAPER_START_SCRIPT.to_string();
            let mut regular_paper_contents = REGULAR_PAPER_START_SCRIPT.to_string();
            if file_path.file_name().unwrap() == "start.sh" {
                aikars_flags_contents.insert_str(0, "#!/bin/bash\n");
                regular_paper_contents.insert_str(0, "#!/bin/bash\n");
            }
            let mut start_script = File::create(file_path)?;
            if aikars_flags {
                start_script.write_all(aikars_flags_contents.as_bytes())?;
            } else {
                start_script.write_all(regular_paper_contents.as_bytes())?;
            }
        }
    }
    Ok(())
}
