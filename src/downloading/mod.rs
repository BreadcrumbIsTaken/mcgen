use serde::{Deserialize, Serialize};

pub mod bungeecord;
pub mod paper;
pub mod plugins;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildData {
    pub artifacts: Option<Vec<Artifact>>,
    pub full_display_name: Option<String>,
    #[serde(rename = "number")]
    pub build: Option<u32>,
    pub url: Option<String>,
    pub versions: Option<Vec<String>>,
    pub builds: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub file_name: String,
    pub relative_path: String,
}
