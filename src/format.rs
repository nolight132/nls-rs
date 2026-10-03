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

pub fn format_permissions(permissions: std::fs::Permissions) -> String {
    format!("{:o}", permissions.mode() & 0o777)
}
