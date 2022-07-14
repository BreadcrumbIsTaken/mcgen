use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VersionData {
    pub server: Option<Vec<HashMap<String, ServerVersionDetails>>>,
    pub plugins: Option<Vec<HashMap<String, PluginVersionDetails>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerVersionDetails {
    pub build: u32,
    // This is a String and not a f64 because there could be more than one decimal place in Minecraft versions, such as 1.18.2
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PluginVersionDetails {
    pub build: u32,
    pub file_name: Option<String>,
    pub url: String,
}

pub fn read_version_file(path: &Path) -> Result<VersionData, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    File::options()
        .read(true)
        .open(path)?
        .read_to_string(&mut contents)?;

    let version_data: VersionData = serde_yaml::from_str(&contents)?;

    Ok(version_data)
}
