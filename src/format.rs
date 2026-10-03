use std::os::unix::fs::PermissionsExt;

use crate::{cli::Args, list::DirEntry};

const PADDING: usize = 3;
const DEFAULT_COLUMNS: &[Column] = &[
    Column {
        kind: ColumnKind::Name,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::Kind,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::Size,
        alignment: Alignment::Right,
        width: 0,
    },
    Column {
        kind: ColumnKind::ModifiedTime,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::Permissions,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::Owner,
        alignment: Alignment::Left,
        width: 0,
    },
];

#[derive(Default)]
#[allow(dead_code)]
pub struct FormatOptions {
    all: bool,
    one: bool,
    plain: bool,
    long: bool,
}

impl FormatOptions {
    pub fn from(args: &Args) -> Self {
        Self {
            all: args.all,
            one: args.one || args._1,
            plain: args.plain,
            long: args.long || args.all,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Column {
    kind: ColumnKind,
    alignment: Alignment,
    width: usize,
}

impl Column {
    pub fn value(&self, entry: &DirEntry) -> String {
        match self.kind {
            ColumnKind::Index => "0".to_string(),
            ColumnKind::Name => entry.name().to_string(),
            ColumnKind::Kind => format_kind(entry.kind()).to_string(),
            ColumnKind::Size => format_size(entry.size()).to_string(),
            ColumnKind::AccessTime => format_time(entry.access_time()).to_string(),
            ColumnKind::ModifiedTime => format_time(entry.modified_time()).to_string(),
            ColumnKind::CreatedTime => format_time(entry.created_time()).to_string(),
            ColumnKind::Permissions => format_permissions(entry.permissions()).to_string(),
            ColumnKind::Owner => entry.owner().to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum ColumnKind {
    Index,
    Name,
    Kind,
    Size,
    AccessTime,
    ModifiedTime,
    CreatedTime,
    Permissions,
    Owner,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Alignment {
    Left,
    Right,
    Center,
}

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

    // TODO: fix "1 minutes ago" situations
    match ago {
        0..MINUTE => "just now".to_string(),
        MINUTE..HOUR => format!("{} minutes ago", ago / MINUTE),
        HOUR..DAY => format!("{} hours ago", ago / HOUR),
        DAY..WEEK => format!("{} days ago", ago / DAY),
        WEEK..MONTH => format!("{} weeks ago", ago / WEEK),
        MONTH..YEAR => format!("{} months ago", ago / MONTH),
        YEAR.. => format!("{} years ago", ago / YEAR),
    }
}

pub fn format_permissions(permissions: std::fs::Permissions) -> String {
    format!("{:o}", permissions.mode() & 0o777)
}

pub fn format_table(entries: Vec<DirEntry>, options: &FormatOptions) -> String {
    let entries = filter_entries(entries, options);
    let mut table = String::new();
    let mut columns = DEFAULT_COLUMNS.to_vec();
    columns = filter_columns(columns, options);

    // TODO: optimize
    for entry in &entries {
        for col in &mut columns {
            col.width = col.width.max(col.value(&entry).len());
        }
    }
    for entry in &entries {
        for col in &columns {
            let str_val = col.value(&entry);
            let formatted = if col.alignment == Alignment::Right {
                format!("{:>width$}", str_val, width = col.width)
            } else {
                format!("{:<width$}", str_val, width = col.width)
            };

            table.push_str(&formatted);
            for _ in 0..PADDING {
                table.push(' ');
            }
        }
        if !options.one {
            table.push('\n');
        }
    }
    if options.one {
        table.push('\n');
    }
    table
}

fn filter_columns(columns: Vec<Column>, options: &FormatOptions) -> Vec<Column> {
    if !options.all && !options.long {
        columns
            .into_iter()
            .filter(|col| matches!(col.kind, ColumnKind::Name | ColumnKind::Size))
            .collect()
    } else {
        columns
    }
}

fn filter_entries(e: Vec<DirEntry>, options: &FormatOptions) -> Vec<DirEntry> {
    e.into_iter()
        .filter(|entry| {
            if options.all {
                true
            } else {
                !entry.name().starts_with(".")
            }
        })
        .collect()
}
