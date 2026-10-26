use serde::{Deserialize, Serialize};

use crate::{
    cli::Args,
    format::{PermissionFormat, TimeFormat},
    list::Property,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub format: FormatConfig,
    pub icons: IconsConfig,
}

impl Config {
    pub fn load() -> Self {
        todo!()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct FormatConfig {
    pub permission_format: PermissionFormat,
    pub time_format: TimeFormat,
}

#[derive(Serialize, Deserialize)]
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
    pub permission_format: PermissionFormat,
    pub icons: bool,
    pub version: bool,
}

impl Options {
    pub fn from(args: &Args, config: &Config) -> Self {
        Self {
            all: args.all,
            long: args.long || args.all,
            time_format: config.format.time_format,
            permission_format: config.format.permission_format,
            icons: config.icons.enabled,
            version: args.version,
        }
    }

    pub fn needs_metadata(&self, properties: &[Property]) -> bool {
        properties.contains(&Property::Size)
            || properties.contains(&Property::Permissions)
            || properties.contains(&Property::ModifiedTime)
            || properties.contains(&Property::AccessTime)
            || properties.contains(&Property::CreatedTime)
            || properties.contains(&Property::Kind)
            || properties.contains(&Property::Owner)
    }
}
