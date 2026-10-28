use owo_colors::{Style, Styled};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Header, // blue + bold
    Property(PropertyKind),
    Entry(EntryKind),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    GitUntracked, // bright green
    GitIgnored,   // bright black
    GitDirty,     // yellow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    Size,         // cyan
    Owner,        // white
    Permissions,  // white
    AccessTime,   // magenta
    ModifiedTime, // magenta
    CreatedTime,  // magenta
    Kind,         // bright black
}

pub fn colorize<D: Display>(d: D, kind: ElementKind) -> Styled<D> {
    style_for(kind).style(d)
}

fn style_for(kind: ElementKind) -> Style {
    match kind {
        ElementKind::Header => Style::new().blue().bold(),
        ElementKind::Entry(EntryKind::Directory) => Style::new().blue(),
        ElementKind::Entry(EntryKind::File) => Style::new(),
        ElementKind::Entry(EntryKind::Symlink) => Style::new(),
        ElementKind::Entry(EntryKind::GitUntracked) => Style::new().bright_green(),
        ElementKind::Entry(EntryKind::GitIgnored) => Style::new().bright_black(),
        ElementKind::Entry(EntryKind::GitDirty) => Style::new().yellow(),
        ElementKind::Property(PropertyKind::Size) => Style::new().cyan(),
        ElementKind::Property(PropertyKind::Permissions) => Style::new().white(),
        ElementKind::Property(PropertyKind::Owner) => Style::new().white(),
        ElementKind::Property(PropertyKind::Kind) => Style::new().bright_black(),
        ElementKind::Property(PropertyKind::CreatedTime) => Style::new().magenta(),
        ElementKind::Property(PropertyKind::AccessTime) => Style::new().magenta(),
        ElementKind::Property(PropertyKind::ModifiedTime) => Style::new().magenta(),
    }
}
