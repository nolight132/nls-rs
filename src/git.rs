use std::path::{Path, PathBuf};

use std::collections::HashMap;

use git2::Statuses;

pub struct GitStatuses(HashMap<PathBuf, GitStatus>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitStatus {
    pub worktree: WorktreeStatus,
    pub index: IndexStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeStatus {
    Clean,
    Dirty,
    Ignored,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexStatus {
    Clean,
    Dirty,
    Untracked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifiedType {
    Index,
    Worktree,
    Both,
}

impl GitStatuses {
    pub fn collect(path: &PathBuf) -> Option<Self> {
        let repo = git2::Repository::discover(path).ok()?;

        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(true)
            .recurse_untracked_dirs(false)
            .recurse_ignored_dirs(false);

        let statuses = repo.statuses(Some(&mut opts)).ok()?;
        Some(GitStatuses::from(statuses))
    }

    pub fn get(&self, path: &Path) -> Option<&GitStatus> {
        self.0.get(path)
    }
}

impl From<Statuses<'_>> for GitStatuses {
    fn from(st: Statuses) -> Self {
        let statuses = HashMap::default();

        st.into_iter().fold(Self(statuses), |mut acc, s| {
            let path = PathBuf::from(s.path().unwrap());
            dbg!(&path);
            acc.0.insert(
                path,
                GitStatus {
                    worktree: WorktreeStatus::Dirty,
                    index: IndexStatus::Clean,
                },
            );
            acc
        })
    }
}
