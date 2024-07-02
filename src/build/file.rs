use std::{ffi::OsStr, fs::read_to_string, path::Path};

use serde::de::DeserializeOwned;
use thiserror::Error;

use serde_json as json;
use serde_yaml as yaml;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unsupported file extension")]
    UnsupportedExtension,
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse json: {0}")]
    Json(#[from] json::Error),
    #[error("Failed to parse toml: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Failed to parse yaml: {0}")]
    Yaml(#[from] yaml::Error),
}

pub fn parse<T>(path: &str) -> Result<T, ParseError>
where
    T: DeserializeOwned,
{
    let Some(extension) = Path::new(path).extension().and_then(OsStr::to_str) else {
        return Err(ParseError::UnsupportedExtension);
    };

    let content = read_to_string(path)?;

    Ok(match extension {
        "json" => json::from_str(&content)?,
        "toml" => toml::from_str(&content)?,
        "yaml" | "yml" => yaml::from_str(&content)?,
        _ => return Err(ParseError::UnsupportedExtension),
    })
}
