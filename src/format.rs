//! Utility module providing functions for formatting file system data.

use std::{os::unix::fs::PermissionsExt, time::SystemTime};

use chrono::{DateTime, Duration, Utc};

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

fn format_plural(count: isize, unit_str: &str) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("{} {}{} ago", count, unit_str, plural)
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TimeFormat {
    #[default]
    Relative,
    Absolute,
}

pub fn format_time(time: SystemTime, format: TimeFormat) -> String {
    if let TimeFormat::Absolute = format {
        let time: DateTime<Utc> = time.into();
        return time.format("%b%e %H:%M").to_string();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PermissionFormat {
    Octal,
    #[default]
    Symbolic,
}

pub fn format_permissions(permissions: std::fs::Permissions, format: PermissionFormat) -> String {
    match format {
        PermissionFormat::Octal => format!("{:o}", permissions.mode() & 0o777),
        PermissionFormat::Symbolic => format_symbolic_permissions(permissions),
    }
}

fn format_symbolic_permissions(permissions: std::fs::Permissions) -> String {
    let mode = permissions.mode() & 0o777;
    let mut result = String::new();

    result.push_str(&format_symbolic_permission(mode, 0o400, 'r'));
    result.push_str(&format_symbolic_permission(mode, 0o200, 'w'));
    result.push_str(&format_symbolic_permission(mode, 0o100, 'x'));
    result.push_str(&format_symbolic_permission(mode, 0o040, 'r'));
    result.push_str(&format_symbolic_permission(mode, 0o020, 'w'));
    result.push_str(&format_symbolic_permission(mode, 0o010, 'x'));
    result.push_str(&format_symbolic_permission(mode, 0o004, 'r'));
    result.push_str(&format_symbolic_permission(mode, 0o002, 'w'));
    result.push_str(&format_symbolic_permission(mode, 0o001, 'x'));
    result
}

fn format_symbolic_permission(mode: u32, mask: u32, char: char) -> String {
    match mode & mask {
        0 => "-".to_string(),
        _ => char.to_string(),
    }
}
