use std::borrow::Cow;

use owo_colors::Styled;
use unicode_width::UnicodeWidthStr;

use crate::{
    color::{EntryKind, PropertyKind, UIKind, colorize},
    config::Options,
    format::{format_kind, format_permissions, format_size, format_time},
    icons,
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
    ) -> Option<(Styled<Cow<'a, str>>, usize)> {
        match self {
            Column::Index => Some(colorize((row + 1).to_string().into(), UIKind::Header)),
            Column::Property(property) => property_value(property, entry, options).map(|v| v),
        }
        .map(|v: Styled<Cow<'a, str>>| {
            let w = v.inner().width();
            (v, w)
        })
    }

    pub fn property(&self) -> Option<Property> {
        match self {
            Column::Index => None,
            Column::Property(property) => Some(*property),
        }
    }

    pub fn header(&self) -> Styled<Cow<'static, str>> {
        match self {
            Column::Index => colorize("#".into(), UIKind::Header),
            Column::Property(property) => colorize(property.header().into(), UIKind::Header),
        }
    }
}

impl From<Property> for Column {
    fn from(p: Property) -> Self {
        Self::Property(p)
    }
}

pub fn columns(options: &Options) -> Vec<Column> {
    use Property::*;

    // Property columns
    let mut props = vec![Name, Size, ModifiedTime];
    if options.long {
        props.extend([Permissions, Owner]);
    }

    // Virtual columns
    [Column::Index]
        .into_iter()
        .chain(props.into_iter().map(Column::Property))
        .collect()
}

fn property_value<'a>(
    property: &Property,
    entry: &'a Entry,
    options: &Options,
) -> Option<Styled<Cow<'a, str>>> {
    match property {
        Property::Name => {
            let name = entry.name();
            let kind = entry.kind();

            match kind.is_some_and(|k| k.is_dir()) {
                true => {
                    let icon = if options.icons { icons::dir_icon() } else { "" };
                    Some(colorize(
                        format!("{icon}{}", name).into(),
                        UIKind::Entry(EntryKind::Directory),
                    ))
                }
                false => {
                    let icon = if options.icons {
                        icons::get_icon(&name)
                    } else {
                        ""
                    };
                    Some(colorize(
                        format!("{icon}{}", name).into(),
                        UIKind::Entry(EntryKind::File),
                    ))
                }
            }
        }
        Property::Kind => entry
            .kind()
            .map(|k| colorize(format_kind(k).into(), UIKind::Property(PropertyKind::Kind))),
        Property::Size => entry
            .size()
            .map(|s| colorize(format_size(s).into(), UIKind::Property(PropertyKind::Size))),
        Property::AccessTime => entry.access_time().map(|t| {
            colorize(
                format_time(t, options.time_format).into(),
                UIKind::Property(PropertyKind::AccessTime),
            )
        }),
        Property::ModifiedTime => entry.modified_time().map(|t| {
            colorize(
                format_time(t, options.time_format).into(),
                UIKind::Property(PropertyKind::ModifiedTime),
            )
        }),
        Property::CreatedTime => entry.created_time().map(|t| {
            colorize(
                format_time(t, options.time_format).into(),
                UIKind::Property(PropertyKind::CreatedTime),
            )
        }),
        Property::Owner => entry
            .owner()
            .map(|o| colorize(o.into(), UIKind::Property(PropertyKind::Owner))),
        Property::Permissions => entry.permissions().map(|p| {
            colorize(
                format_permissions(p, options.permission_format).into(),
                UIKind::Property(PropertyKind::Permissions),
            )
        }),
    }
}
