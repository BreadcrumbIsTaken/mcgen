use std::path::Path;
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};

pub async fn generate_version_file(
    path: &Path,
    contents: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let versions_file = File::create(path.join("mcgen.txt")).await;
    versions_file?.write_all(contents.as_bytes()).await?;

    Ok(())
}

pub async fn write_to_plugin_version_file(
    path: &Path,
    mut contents: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.join("mcgen.txt").exists() {
        let mut versions_file = File::create(path.join("mcgen.txt")).await?;
        versions_file.write_all(b"plugins:\n").await?;
    }
    let mut versions_file = OpenOptions::new()
        .append(true)
        .open(path.join("mcgen.txt"))
        .await?;
    contents.push('\n');
    versions_file.write_all(contents.as_bytes()).await?;

    Ok(())
}
