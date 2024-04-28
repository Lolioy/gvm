use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

pub const CURRENT_VERSION_PATH: &str = ".current_version";
pub const VERSION_TAG: &str = "_version_";
pub const LOCAL_VERSION_PREFIX: &str = "local";
pub const MORE_VERSION_PREFIX: &str = "more";

pub fn set_current_version(version: &str) -> Result<()> {
    let mut file = fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(PathBuf::from(CURRENT_VERSION_PATH))?;
    file.write_all(version.as_bytes())?;
    Ok(())
}

pub fn get_current_version() -> Result<String> {
    let filepath = PathBuf::from(CURRENT_VERSION_PATH);
    if !filepath.exists() { return Ok("".to_string()); }

    let mut file = fs::File::open(filepath)?;
    let mut version = String::new();
    file.read_to_string(&mut version)?;
    Ok(version)
}

pub fn get_local_versions() -> Vec<String> {
    vec![
        "go1.21".to_string(),
    ]
}

pub fn get_more_versions() -> Vec<String> {
    vec![
        "go1.21".to_string(),
        "go1.20".to_string(),
    ]
}