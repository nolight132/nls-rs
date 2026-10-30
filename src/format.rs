//! Utility module providing functions for formatting file system data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{os::unix::fs::PermissionsExt, time::SystemTime};

pub fn format_size(size: u64) -> String {
    let mut size = size as f64;
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    let prec = if unit < 1 { 0 } else { 1 };
    format!("{size:.prec$} {}", units[unit])
}

pub fn format_kind(kind: std::fs::FileType) -> &'static str {
    if kind.is_dir() {
        "dir"
    } else if kind.is_file() {
        "file"
    } else if kind.is_symlink() {
        "sym"
    } else {
        "other"
    }
}

fn format_plural(count: isize, unit: &str) -> String {
    if count == 1 {
        format!("a {unit} ago")
    } else {
        format!("{count} {unit}s ago")
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    #[default]
    Relative,
    Absolute,
}

pub fn format_time(time: SystemTime, format: TimeFormat) -> String {
    if let TimeFormat::Absolute = format {
        let time: DateTime<Utc> = time.into();
        return time.format("%b %e %H:%M").to_string();
    }

    let time = DateTime::from(time);
    let passed = date_component::date_component::calculate(&time, &Utc::now());

    [
        (passed.year, "year"),
        (passed.month, "month"),
        (passed.day, "day"),
        (passed.hour, "hour"),
        (passed.minute, "minute"),
    ]
    .into_iter()
    .find(|&(n, _)| n > 0)
    .map(|(n, u)| format_plural(n, u))
    .unwrap_or_else(|| "just now".to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionsFormat {
    Octal,
    #[default]
    Symbolic,
}

pub fn format_permissions(permissions: std::fs::Permissions, format: PermissionsFormat) -> String {
    match format {
        PermissionsFormat::Octal => format!("{:o}", permissions.mode() & 0o777),
        PermissionsFormat::Symbolic => format_symbolic_permissions(permissions),
    }
}

fn format_symbolic_permissions(permissions: std::fs::Permissions) -> String {
    let mode = permissions.mode();
    let mut result = String::new();

    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        0o020000 => 'c',
        0o060000 => 'b',
        0o010000 => 'p',
        0o140000 => 's',
        _ => '-',
    };
    result.push(file_type);

    [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ]
    .into_iter()
    .for_each(|(mask, char)| {
        result.push(match mode & mask {
            0 => '-',
            _ => char,
        });
    });
    result
}
