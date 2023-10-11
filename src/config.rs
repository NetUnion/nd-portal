use std::fs::File;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigItem {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) host: String,
    pub(crate) ip: Option<String>,
    pub(crate) interface: Option<String>,
}

pub(crate) fn read_from_file(path: &str) -> Result<Vec<ConfigItem>> {
    let file = File::open(path).context("Failed to open config file.")?;
    let config: Vec<ConfigItem> =
        serde_json::from_reader(file).context("Failed to parse config file.")?;
    for item in &config {
        if item.username.is_empty() {
            return Err(anyhow::anyhow!("Empty username."));
        }
        if item.password.is_empty() {
            return Err(anyhow::anyhow!("Empty password."));
        }
        if item.host.is_empty() {
            return Err(anyhow::anyhow!("Empty host."));
        }
        if item.ip.is_some() && item.interface.is_some() {
            return Err(anyhow::anyhow!("Both ip and interface are specified."));
        }
        if item.ip.is_none() && item.interface.is_none() && config.len() > 1 {
            return Err(anyhow::anyhow!("Neither ip nor interface is specified."));
        }
    }
    Ok(config)
}
