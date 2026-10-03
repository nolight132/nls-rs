use std::{
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};
use uzers::get_user_by_uid;

#[derive(Debug)]
pub struct DirEntry {
    name: String,
    path: PathBuf,
    kind: fs::FileType,
    size: u64,
    permissions: fs::Permissions,
    access_time: std::time::SystemTime,
    modified_time: std::time::SystemTime,
    created_time: std::time::SystemTime,
    owner: String,
}

#[allow(dead_code)]
impl DirEntry {
    pub fn new(
        name: String,
        path: PathBuf,
        kind: fs::FileType,
        size: u64,
        permissions: fs::Permissions,
        access_time: std::time::SystemTime,
        modified_time: std::time::SystemTime,
        created_time: std::time::SystemTime,
        owner: String,
    ) -> Self {
        Self {
            name,
            path,
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
        &self.name
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn kind(&self) -> fs::FileType {
        self.kind
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn permissions(&self) -> fs::Permissions {
        self.permissions.clone()
    }
    pub fn access_time(&self) -> std::time::SystemTime {
        self.access_time
    }
    pub fn modified_time(&self) -> std::time::SystemTime {
        self.modified_time
    }
    pub fn created_time(&self) -> std::time::SystemTime {
        self.created_time
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
}

pub fn stat_dir(path: &Path) -> std::io::Result<Vec<DirEntry>> {
    std::fs::read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            let metadata = entry.metadata()?;
            Ok(DirEntry::new(
                entry.file_name().to_string_lossy().into_owned(),
                entry.path(),
                metadata.file_type(),
                metadata.len(),
                metadata.permissions(),
                metadata
                    .accessed()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                metadata
                    .modified()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                metadata
                    .created()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                match get_user_by_uid(metadata.uid()) {
                    Some(user) => user.name().to_string_lossy().to_string(),
                    None => "unknown".to_string(),
                },
            ))
        })
        .collect()
}
