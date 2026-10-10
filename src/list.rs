use std::{
    fs::{self, DirEntry, ReadDir},
    io,
    os::unix::fs::MetadataExt,
    path::Path,
    time::UNIX_EPOCH,
};

use uzers::{Users, UsersCache};

use crate::{
    cli::Args,
    config::{Config, Options},
    format::{PermissionFormat, TimeFormat},
};

#[derive(Debug)]
pub struct Entry {
    name: String,
    kind: Option<fs::FileType>,
    size: Option<u64>,
    permissions: Option<fs::Permissions>,
    access_time: Option<std::time::SystemTime>,
    modified_time: Option<std::time::SystemTime>,
    created_time: Option<std::time::SystemTime>,
    owner: Option<String>,
}

impl Entry {
    pub fn new(
        name: String,
        kind: Option<fs::FileType>,
        size: Option<u64>,
        permissions: Option<fs::Permissions>,
        access_time: Option<std::time::SystemTime>,
        modified_time: Option<std::time::SystemTime>,
        created_time: Option<std::time::SystemTime>,
        owner: Option<String>,
    ) -> Self {
        Self {
            name,
            kind,
            size,
            permissions,
            access_time,
            modified_time,
            created_time,
            owner,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn kind(&self) -> Option<fs::FileType> {
        self.kind
    }
    pub fn size(&self) -> Option<u64> {
        self.size
    }
    pub fn permissions(&self) -> Option<fs::Permissions> {
        self.permissions.clone()
    }
    pub fn access_time(&self) -> Option<std::time::SystemTime> {
        self.access_time
    }
    pub fn modified_time(&self) -> Option<std::time::SystemTime> {
        self.modified_time
    }
    pub fn created_time(&self) -> Option<std::time::SystemTime> {
        self.created_time
    }
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Property {
    Name,
    Kind,
    Size,
    Permissions,
    AccessTime,
    ModifiedTime,
    CreatedTime,
    Owner,
}

pub fn stat_dir(
    path: &Path,
    properties: &[Property],
    options: &Options,
) -> std::io::Result<Vec<Entry>> {
    let cache = UsersCache::new();

    let entries = filter_entries(std::fs::read_dir(path)?, options)?
        .into_iter()
        .map(|entry| {
            let needs_metadata = options.needs_metadata(properties);
            let metadata = if needs_metadata {
                Some(entry.metadata()?)
            } else {
                None
            };
            Ok(Entry::new(
                entry.file_name().to_string_lossy().into_owned(),
                metadata.as_ref().map(|m| m.file_type()),
                metadata.as_ref().map(|m| m.size()),
                metadata.as_ref().map(|m| m.permissions()),
                metadata
                    .as_ref()
                    .map(|m| m.accessed().unwrap_or(UNIX_EPOCH)),
                metadata
                    .as_ref()
                    .map(|m| m.modified().unwrap_or(UNIX_EPOCH)),
                metadata.as_ref().map(|m| m.created().unwrap_or(UNIX_EPOCH)),
                metadata
                    .as_ref()
                    .map(|m| match cache.get_user_by_uid(m.uid()) {
                        Some(user) => user.name().to_string_lossy().into_owned(),
                        None => "unknown".to_string(),
                    }),
            ))
        })
        .collect::<std::io::Result<Vec<_>>>()?;

    Ok(entries)
}

fn filter_entries(e: ReadDir, options: &Options) -> io::Result<Vec<DirEntry>> {
    e.filter(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return true,
        };
        match options.all {
            true => true,
            false => !entry.file_name().to_string_lossy().starts_with("."),
        }
    })
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stat_dir_empty() {
        let path = Path::new("/tmp/empty_test");
        fs::create_dir(path).ok();

        let properties = &[Property::Name];
        let entries = stat_dir(path, properties, &Options::default()).unwrap();

        assert!(entries.is_empty());
        fs::remove_dir(path).ok();
    }

    #[test]
    fn stat_dir_non_empty() {
        let path = Path::new("/tmp/non_empty_test");
        fs::File::create(path.join("test.txt")).ok();
        fs::create_dir(path).ok();

        let properties = &[Property::Name];
        let entries = stat_dir(path, properties, &Options::default()).unwrap();

        assert!(!entries.is_empty());
        fs::remove_dir(path).ok();
    }

    #[test]
    fn stat_dir_non_existent() {
        let path = Path::new("/tmp/non_existent_test");
        let properties = &[Property::Name];
        let entries = stat_dir(path, properties, &Options::default());

        assert!(entries.is_err());
    }
}
