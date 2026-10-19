use owo_colors::{Style, Styled};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIKind {
    Heading, // blue
    Header,  // blue + bold
    Border,
    Property(PropertyKind),
    Entry(EntryKind),
}

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
    Size,     // cyan
    Modified, // magenta
    Error,    // red
    Empty,    // bright black
}

pub fn color<D: Display>(d: D, kind: UIKind) -> Styled<D> {
    style_for(kind).style(d)
}

fn style_for(kind: UIKind) -> Style {
    match kind {
        UIKind::Heading => Style::new().blue(),
        UIKind::Header => Style::new().blue().bold(),
        UIKind::Property(PropertyKind::Size) => Style::new().cyan(),
        UIKind::Property(PropertyKind::Modified) => Style::new().magenta(),
        UIKind::Property(PropertyKind::Error) => Style::new().red(),
        UIKind::Property(PropertyKind::Empty) => Style::new().bright_black(),
        UIKind::Entry(EntryKind::GitUntracked) => Style::new().bright_green(),
        UIKind::Entry(EntryKind::GitIgnored) => Style::new().bright_black(),
        UIKind::Entry(EntryKind::GitDirty) => Style::new().yellow(),
        UIKind::Border => Style::new().bright_black(),
        UIKind::Entry(EntryKind::Directory) | UIKind::Entry(EntryKind::File) => Style::new(),
    }
}
