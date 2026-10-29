use std::path::{Path, PathBuf};

use std::collections::HashMap;

use git2::ErrorClass::Worktree;
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
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexStatus {
    Clean,
    Modified,
    Deleted,
}

impl GitStatuses {
    pub fn collect(path: &PathBuf) -> Option<Self> {
        let repo = git2::Repository::discover(path).ok()?;
        let root = repo.workdir()?;

        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(true)
            .recurse_untracked_dirs(false)
            .recurse_ignored_dirs(false);

        let statuses = repo.statuses(Some(&mut opts)).ok()?;
        Some(GitStatuses::from_statuses(statuses, root))
    }

    pub fn get(&self, path: &Path) -> Option<&GitStatus> {
        self.0.get(path)
    }
}

impl GitStatuses {
    fn from_statuses(st: Statuses, root: &Path) -> Self {
        let statuses = HashMap::default();

        st.into_iter().fold(Self(statuses), |mut acc, s| {
            if let Ok(path) = s.path() {
                let path = root.join(PathBuf::from(path));
                let status: GitStatus = (&s).into();

                for parent in path.ancestors().skip(1).take_while(|p| *p != root) {
                    acc.0
                        .entry(parent.to_path_buf())
                        .or_insert_with(|| status.clone());
                }
                acc.0.insert(path, status);
            }

            acc
        })
    }
}

impl GitStatus {
    pub fn is_dirty(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Modified)
    }

    pub fn is_untracked(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Untracked)
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Ignored)
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self.worktree, WorktreeStatus::Deleted)
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
        } else if s.status().is_wt_deleted() {
            WorktreeStatus::Deleted
        } else {
            WorktreeStatus::Clean
        };
        Self {
            worktree,
            // TODO: handle index status
            index: IndexStatus::Clean,
        }
    }
}
