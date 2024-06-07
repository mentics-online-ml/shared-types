use std::path::PathBuf;
use anyhow::Context;

pub fn artifacts_dir() -> anyhow::Result<PathBuf> {
    let path = data_dir()?.join("models").join("oml");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn data_dir() -> anyhow::Result<PathBuf> {
    let h = home::home_dir().with_context(|| "Could not get user home directory")?;
    Ok(h.join("data"))
}
