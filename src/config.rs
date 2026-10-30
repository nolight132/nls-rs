use crate::{
    cli::Args,
    format::{PermissionsFormat, TimeFormat},
    list::Property,
};
use anyhow::{Context, Result};
use etcetera::{BaseStrategy, choose_base_strategy};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub format: FormatConfig,
    pub icons: IconsConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let strategy = choose_base_strategy()
            .ok()
            .context("failed to choose base strategy")?;
        let path = PathBuf::from(strategy.config_dir())
            .join("nls")
            .join("config.toml");
        let config = fs::read_to_string(&path)
            .ok()
            .context("failed to read config file")?;
        let config = match toml::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return Err(anyhow::anyhow!("failed to parse {}", path.display()));
            }
        };

        Some(config).with_context(|| format!("failed to parse {}", path.display()))
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    pub permissions: PermissionsFormat,
    pub time: TimeFormat,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IconsConfig {
    pub enabled: bool,
}

impl Default for IconsConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Default)]
pub struct Options {
    pub all: bool,
    pub long: bool,
    pub time_format: TimeFormat,
    pub permission_format: PermissionsFormat,
    pub icons: bool,
    pub version: bool,
}

impl Options {
    pub fn from(args: &Args, config: &Config) -> Self {
        Self {
            all: args.all,
            long: args.long || args.all,
            time_format: config.format.time,
            permission_format: config.format.permissions,
            icons: config.icons.enabled,
            version: args.version,
        }
    }

    pub fn needs_metadata(&self, properties: &[Property]) -> bool {
        properties.contains(&Property::Size)
            || properties.contains(&Property::Kind)
            || properties.contains(&Property::Permissions)
            || properties.contains(&Property::ModifiedTime)
            || properties.contains(&Property::AccessTime)
            || properties.contains(&Property::CreatedTime)
            || properties.contains(&Property::Owner)
    }
}
