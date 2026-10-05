use crate::{
    format::{PermissionFormat, TimeFormat},
    view::Column,
};

pub struct Config {
    pub short_cols: Vec<Column>,
    pub long_cols: Vec<Column>,

    pub permission_format: PermissionFormat,
    pub time_format: TimeFormat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            short_cols: Vec::new(),
            long_cols: Vec::new(),
            permission_format: PermissionFormat::Octal,
            time_format: TimeFormat::Relative,
        }
    }
}
