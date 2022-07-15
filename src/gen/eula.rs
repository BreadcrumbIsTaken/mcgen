use chrono::prelude::*;
use std::fmt::Write as _;
use std::path::Path;
use std::{fs::OpenOptions, io::Write};

use crate::consts::eula::EULA_ACKNOWLEDGMENT;

pub fn generate_eula(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(path).join("eula.txt");
    let mut eula = OpenOptions::new().create(true).write(true).open(file)?;
    let mut contents = String::new();
    writeln!(contents, "{}", EULA_ACKNOWLEDGMENT)?;

    let dt = Local::now();
    // Immitates the eula generated by Paper with an exception of the timezone. (Instead of an abbriviation like PST it is an UTC offset)
    writeln!(
        contents,
        "#{}",
        &dt.format("%a %b %d %H:%M:%S UTC%Z %Y").to_string()
    )?;
    writeln!(contents, "eula=true")?;

    eula.write_all(contents.as_bytes())?;

    Ok(())
}
