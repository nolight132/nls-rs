use crate::{
    format::{format_kind, format_permissions, format_size, format_time},
    list::{Entry, ListOptions, Property},
};

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Entry>,
}

impl Table {
    pub fn new(entries: Vec<Entry>, columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: entries,
        }
    }

    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn rows(&self) -> &[Entry] {
        &self.rows
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Column {
    Index,
    Property(Property),
}

pub fn columns(options: &ListOptions) -> Vec<Column> {
    let mut columns = vec![
        Column::Index,
        Column::Property(Property::Name),
        Column::Property(Property::Size),
        Column::Property(Property::ModifiedTime),
    ];

    if options.long {
        columns.extend(vec![
            Column::Property(Property::Permissions),
            Column::Property(Property::Owner),
        ]);
    }
    // read from config, fall back to these defaults
    columns
}

impl Column {
    pub fn value(&self, entry: &Entry, row: usize, options: &ListOptions) -> Option<String> {
        match self {
            Column::Index => Some((row + 1).to_string()),
            Column::Property(property) => property_value(*property, entry, options),
        }
    }

    pub fn property(&self) -> Option<Property> {
        match self {
            Column::Index => None,
            Column::Property(property) => Some(*property),
        }
    }
}

fn property_value(property: Property, entry: &Entry, options: &ListOptions) -> Option<String> {
    match property {
        Property::Name => entry.name().map(str::to_owned),
        Property::Kind => entry.kind().map(|k| format_kind(k).to_owned()),
        Property::Size => entry.size().map(format_size),
        Property::AccessTime => entry
            .access_time()
            .map(|t| format_time(t, options.time_format)),
        Property::ModifiedTime => entry
            .modified_time()
            .map(|t| format_time(t, options.time_format)),
        Property::CreatedTime => entry
            .created_time()
            .map(|t| format_time(t, options.time_format)),
        Property::Owner => entry.owner().map(str::to_owned),
        Property::Permissions => entry
            .permissions()
            .map(|p| format_permissions(p, options.permission_format)),
    }
}

impl From<Property> for Column {
    fn from(p: Property) -> Self {
        Self::Property(p)
    }
}
