use std::path::{Path, PathBuf};

use std::collections::HashMap;

use git2::{Status, StatusEntry, Statuses};

pub struct GitStatuses(HashMap<PathBuf, GitStatus>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitStatus {
    worktree: WorktreeStatus,
    index: IndexStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeStatus {
    Clean,
    Modified,
    Untracked,
    Ignored,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexStatus {
    Clean,
    Modified,
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
            acc.0.insert(path, (&s).into());
            acc
        })
    }
}

impl GitStatus {
    pub fn worktree(&self) -> &WorktreeStatus {
        &self.worktree
    }

    pub fn index(&self) -> &IndexStatus {
        &self.index
    }

    pub fn is_dirty(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Modified)
    }

    pub fn is_untracked(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Untracked)
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Ignored)
    }
}

impl From<&StatusEntry<'_>> for GitStatus {
    fn from(s: &StatusEntry<'_>) -> Self {
        let worktree = if s.status().is_wt_new() {
            WorktreeStatus::Untracked
        } else if s.status().is_wt_modified() {
            WorktreeStatus::Modified
        } else if s.status().is_ignored() {
            WorktreeStatus::Ignored
        } else {
            WorktreeStatus::Clean
        };
        Self {
            worktree,
            index: IndexStatus::Clean,
        }
    }
}
