use crate::{
    cli::Args,
    format::{format_kind, format_permissions, format_size, format_time},
    list::DirEntry,
};

#[derive(Default)]
#[allow(dead_code)]
pub struct OutputOptions {
    pub all: bool,
    pub one: bool,
    pub plain: bool,
    pub long: bool,
}

impl OutputOptions {
    pub fn from(args: &Args) -> Self {
        Self {
            all: args.all,
            one: args.one || args._1,
            plain: args.plain,
            long: args.long || args.all,
        }
    }
}

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

pub fn build_table(entries: Vec<DirEntry>, options: &OutputOptions) -> String {
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

fn filter_columns(columns: Vec<Column>, options: &OutputOptions) -> Vec<Column> {
    if !options.all && !options.long {
        columns
            .into_iter()
            .filter(|col| {
                matches!(
                    col.kind,
                    ColumnKind::Name | ColumnKind::Size | ColumnKind::ModifiedTime
                )
            })
            .collect()
    } else {
        columns
    }
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
