use std::borrow::Cow;

use crate::{
    config::Options,
    format::{format_kind, format_permissions, format_size, format_time},
    list::{Entry, Property},
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

#[derive(Clone)]
pub enum Column {
    Index,
    Property(Property),
}

impl Column {
    pub fn value<'a>(
        &self,
        entry: &'a Entry,
        row: usize,
        options: &Options,
    ) -> Option<Cow<'a, str>> {
        match self {
            Column::Index => Some((row + 1).to_string().into()),
            Column::Property(property) => {
                property_value(property, entry, options).map(|v| v.into())
            }
        }
    }

    pub fn property(&self) -> Option<Property> {
        match self {
            Column::Index => None,
            Column::Property(property) => Some(*property),
        }
    }

    pub fn header(&self) -> &str {
        match self {
            Column::Index => "#",
            Column::Property(property) => property.header(),
        }
    }
}

pub fn columns(options: &Options) -> Vec<Column> {
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

fn property_value<'a>(
    property: &Property,
    entry: &'a Entry,
    options: &Options,
) -> Option<Cow<'a, str>> {
    match property {
        Property::Name => Some(entry.name().into()),
        Property::Kind => entry.kind().map(|k| format_kind(k).into()),
        Property::Size => entry.size().map(|s| format_size(s).into()),
        Property::AccessTime => entry
            .access_time()
            .map(|t| format_time(t, options.time_format).into()),
        Property::ModifiedTime => entry
            .modified_time()
            .map(|t| format_time(t, options.time_format).into()),
        Property::CreatedTime => entry
            .created_time()
            .map(|t| format_time(t, options.time_format).into()),
        Property::Owner => entry.owner().map(|o| o.into()),
        Property::Permissions => entry
            .permissions()
            .map(|p| format_permissions(p, options.permission_format).into()),
    }
}

impl From<Property> for Column {
    fn from(p: Property) -> Self {
        Self::Property(p)
    }
}
