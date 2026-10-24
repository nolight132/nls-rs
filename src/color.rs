use owo_colors::{Style, Styled};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIKind {
    Header, // blue + bold
    Property(PropertyKind),
    Entry(EntryKind),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    GitUntracked, // bright green
    GitIgnored,   // bright black
    GitDirty,     // yellow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKind {
    Size,         // cyan
    Owner,        // bright black
    Permissions,  // bright black
    AccessTime,   // bright black
    ModifiedTime, // bright black
    CreatedTime,  // bright black
    Kind,         // bright black
}

pub fn colorize<D: Display>(d: D, kind: UIKind) -> Styled<D> {
    style_for(kind).style(d)
}

fn style_for(kind: UIKind) -> Style {
    match kind {
        UIKind::Header => Style::new().blue().bold(),
        UIKind::Entry(EntryKind::GitUntracked) => Style::new().bright_green(),
        UIKind::Entry(EntryKind::GitIgnored) => Style::new().bright_black(),
        UIKind::Entry(EntryKind::GitDirty) => Style::new().yellow(),
        UIKind::Entry(EntryKind::Directory) => Style::new().blue(),
        UIKind::Entry(EntryKind::File) => Style::new(),
        UIKind::Property(PropertyKind::Size) => Style::new().cyan(),
        UIKind::Property(PropertyKind::Permissions) => Style::new().white(),
        UIKind::Property(PropertyKind::Owner) => Style::new().white(),
        UIKind::Property(PropertyKind::Kind) => Style::new().bright_black(),
        UIKind::Property(PropertyKind::CreatedTime) => Style::new().magenta(),
        UIKind::Property(PropertyKind::AccessTime) => Style::new().magenta(),
        UIKind::Property(PropertyKind::ModifiedTime) => Style::new().magenta(),
    }
}
