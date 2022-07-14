use std::{collections::HashMap, path::Path};

use crate::downloading::plugins::download_plugin;

pub async fn add_plugin_to_existing_server(
    directory: String,
    name: String,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(&directory);

    download_plugin(dir, &HashMap::from([(name, url)])).await?;

    Ok(())
}
