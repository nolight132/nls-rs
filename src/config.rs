use crate::format::{PermissionFormat, TimeFormat};

#[derive(Default)]
pub struct Config {
    pub permission_format: PermissionFormat,
    pub time_format: TimeFormat,
}
