use std::os::unix::fs::PermissionsExt;

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

macro_rules! format_plural {
    ($secs:expr, $unit:expr, $unit_str:expr) => {{
        let count = $secs / $unit;
        let plural = if count == 1 { "" } else { "s" };
        format!("{} {}{} ago", count, $unit_str, plural)
    }};
}

pub fn format_time(time: std::time::SystemTime) -> String {
    let ago = match time.elapsed() {
        Ok(elapsed) => elapsed.as_secs(),
        Err(_) => return "unknown".to_string(),
    };
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * 60;
    const DAY: u64 = HOUR * 24;
    const WEEK: u64 = DAY * 7;
    const MONTH: u64 = WEEK * 4;
    // TODO: count years properly
    const YEAR: u64 = MONTH * 12;

    match ago {
        0..MINUTE => "just now".to_string(),
        MINUTE..HOUR => format_plural!(ago, MINUTE, "minute"),
        HOUR..DAY => format_plural!(ago, HOUR, "hour"),
        DAY..WEEK => format_plural!(ago, DAY, "day"),
        WEEK..MONTH => format_plural!(ago, WEEK, "week"),
        MONTH..YEAR => format_plural!(ago, MONTH, "month"),
        YEAR.. => format_plural!(ago, YEAR, "year"),
    }
}

pub enum PermissionFormat {
    Octal,
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
