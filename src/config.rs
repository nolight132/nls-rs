use crate::{
    cli::Args,
    format::{PermissionFormat, TimeFormat},
    list::Property,
};

#[derive(Default)]
pub struct Config {
    pub permission_format: PermissionFormat,
    pub time_format: TimeFormat,
}

#[derive(Default)]
pub struct Options {
    pub all: bool,
    pub long: bool,
    pub time_format: TimeFormat,
    pub permission_format: PermissionFormat,
    pub version: bool,
}

impl Options {
    pub fn from(args: &Args, config: &Config) -> Self {
        Self {
            all: args.all,
            long: args.long || args.all,
            time_format: config.time_format,
            permission_format: config.permission_format,
            version: args.version,
        }
    }

    pub fn needs_metadata(&self, properties: &[Property]) -> bool {
        properties.contains(&Property::Size)
            || properties.contains(&Property::ModifiedTime)
            || properties.contains(&Property::AccessTime)
            || properties.contains(&Property::CreatedTime)
            || properties.contains(&Property::Kind)
            || properties.contains(&Property::Owner)
    }
}
