use unicode_width::UnicodeWidthStr;

use crate::{
    cli::Args,
    format::{PermissionFormat, format_kind, format_permissions, format_size, format_time},
    list::DirEntry,
};
use std::fmt::Write;

#[derive(Default)]
#[allow(dead_code)]
pub struct OutputOptions {
    pub all: bool,
    pub plain: bool,
    pub long: bool,
}

impl OutputOptions {
    pub fn from(args: &Args) -> Self {
        Self {
            all: args.all,
            plain: args.plain,
            long: args.long || args.all,
        }
    }
}

const PADDING: usize = 1;

const BORDER_TOP_LEFT: &str = "╭";
const BORDER_TOP_RIGHT: &str = "╮";
const BORDER_TOP_MIDDLE: &str = "┬";
const BORDER_BOTTOM_LEFT: &str = "╰";
const BORDER_BOTTOM_RIGHT: &str = "╯";
const BORDER_BOTTOM_MIDDLE: &str = "┴";
const BORDER_HORIZONTAL: &str = "─";
const BORDER_VERTICAL: &str = "│";
const _BORDER_CROSS: &str = "┼";

const COLUMNS: &[Column] = &[
    Column {
        kind: ColumnKind::Index,
        alignment: Alignment::Right,
        width: 0,
    },
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
        kind: ColumnKind::AccessTime,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::ModifiedTime,
        alignment: Alignment::Left,
        width: 0,
    },
    Column {
        kind: ColumnKind::CreatedTime,
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

const SHORT_COLS: &[ColumnKind] = &[
    ColumnKind::Index,
    ColumnKind::Name,
    ColumnKind::Size,
    ColumnKind::ModifiedTime,
];

const LONG_COLS: &[ColumnKind] = &[ColumnKind::Permissions, ColumnKind::Owner];

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Column {
    kind: ColumnKind,
    alignment: Alignment,
    width: usize,
}

impl Column {
    pub fn value(&self, entry: &DirEntry, index: Option<usize>) -> String {
        match self.kind {
            ColumnKind::Index => index.map(|i| i.to_string()).unwrap_or_default(),
            ColumnKind::Name => entry.name().to_string(),
            ColumnKind::Kind => format_kind(entry.kind()).to_string(),
            ColumnKind::Size => format_size(entry.size()).to_string(),
            ColumnKind::AccessTime => format_time(entry.access_time()).to_string(),
            ColumnKind::ModifiedTime => format_time(entry.modified_time()).to_string(),
            ColumnKind::CreatedTime => format_time(entry.created_time()).to_string(),
            ColumnKind::Permissions => {
                format_permissions(entry.permissions(), PermissionFormat::Symbolic).to_string()
            }
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

enum Edge {
    Top,
    Bottom,
}

pub fn build_table(entries: Vec<DirEntry>, options: &OutputOptions) -> String {
    let entries = filter_entries(entries, options);
    let mut table = String::new();
    let mut columns = build_columns(options);

    for (i, entry) in entries.iter().enumerate() {
        for col in &mut columns {
            col.width = col.width.max(UnicodeWidthStr::width(
                col.value(&entry, Some(i + 1)).as_str(),
            ));
        }
    }

    write_edge(&mut table, &columns, Edge::Top);
    for (i, entry) in entries.iter().enumerate() {
        table.push_str(BORDER_VERTICAL);
        for (j, col) in columns.iter().enumerate() {
            for _ in 0..PADDING {
                table.push(' ');
            }
            let str_val = col.value(&entry, Some(i + 1));
            if col.alignment == Alignment::Right {
                write!(table, "{:>width$}", str_val, width = col.width).unwrap();
            } else {
                write!(table, "{:<width$}", str_val, width = col.width).unwrap();
            }

            for _ in 0..PADDING {
                table.push(' ');
            }
            if j < columns.len() - 1 {
                table.push_str(BORDER_VERTICAL);
            }
        }
        table.push_str(BORDER_VERTICAL);
        table.push('\n');
    }
    write_edge(&mut table, &columns, Edge::Bottom);
    table
}

fn write_edge(table: &mut String, columns: &[Column], edge: Edge) {
    match edge {
        Edge::Top => table.push_str(BORDER_TOP_LEFT),
        Edge::Bottom => table.push_str(BORDER_BOTTOM_LEFT),
    }
    for (i, col) in columns.iter().enumerate() {
        for j in 0..(col.width + PADDING * 2) {
            table.push_str(BORDER_HORIZONTAL);
            if j == col.width + PADDING * 2 - 1 {
                if i < columns.len() - 1 {
                    match edge {
                        Edge::Top => table.push_str(BORDER_TOP_MIDDLE),
                        Edge::Bottom => table.push_str(BORDER_BOTTOM_MIDDLE),
                    }
                } else {
                    match edge {
                        Edge::Top => table.push_str(BORDER_TOP_RIGHT),
                        Edge::Bottom => table.push_str(BORDER_BOTTOM_RIGHT),
                    }
                }
            }
        }
    }
    table.push('\n');
}

fn build_columns(options: &OutputOptions) -> Vec<Column> {
    COLUMNS
        .iter()
        .copied()
        .filter(|col| {
            SHORT_COLS.contains(&col.kind) || (options.long && LONG_COLS.contains(&col.kind))
        })
        .collect()
}

fn filter_entries(e: Vec<DirEntry>, options: &OutputOptions) -> Vec<DirEntry> {
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
