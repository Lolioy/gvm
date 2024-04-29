use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

pub const CURRENT_VERSION_PATH: &str = ".current_version";

pub fn set_current_version(version: &str) -> Result<()> {
    let mut file = fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(PathBuf::from(CURRENT_VERSION_PATH))?;
    file.write_all(version.as_bytes())?;
    Ok(())
}

pub fn get_current_version() -> Result<Option<String>> {
    let filepath = PathBuf::from(CURRENT_VERSION_PATH);
    if !filepath.exists() { return Ok(None); }

    let mut file = fs::File::open(filepath)?;
    let mut version = String::new();
    file.read_to_string(&mut version)?;
    Ok(Some(version))
}

pub fn get_version_list() -> Vec<String> {
    vec![
        "go1.21".to_string(),
        "go1.20".to_string(),
        "go1.19".to_string(),
    ]
}
