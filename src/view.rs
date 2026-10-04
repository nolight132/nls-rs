use crate::{
    format::{PermissionFormat, format_kind, format_permissions, format_size, format_time},
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
        Column::Property(Property::Permissions),
    ];
    // read from config, fall back to these defaults
    columns
}

impl Column {
    pub fn value(&self, entry: &Entry, row: usize) -> Option<String> {
        match self {
            Column::Index => Some((row + 1).to_string()),
            Column::Property(property) => property_value(*property, entry),
        }
    }

    pub fn property(&self) -> Option<Property> {
        match self {
            Column::Index => None,
            Column::Property(property) => Some(*property),
        }
    }
}

fn property_value(property: Property, entry: &Entry) -> Option<String> {
    match property {
        Property::Name => entry.name().map(str::to_owned),
        Property::Kind => entry.kind().map(|k| format_kind(k).to_owned()),
        Property::Size => entry.size().map(format_size),
        Property::AccessTime => entry.access_time().map(format_time),
        Property::ModifiedTime => entry.modified_time().map(format_time),
        Property::CreatedTime => entry.created_time().map(format_time),
        Property::Owner => entry.owner().map(str::to_owned),
        Property::Permissions => entry
            .permissions()
            .map(|p| format_permissions(p, PermissionFormat::Symbolic)),
    }
}

impl From<Property> for Column {
    fn from(p: Property) -> Self {
        Self::Property(p)
    }
}
