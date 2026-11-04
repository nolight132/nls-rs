use git2::{StatusEntry, Statuses};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
    Added,
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

        st.iter().fold(Self(statuses), |mut acc, s| {
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
    pub fn worktree(&self) -> &WorktreeStatus {
        &self.worktree
    }

    pub fn index(&self) -> &IndexStatus {
        &self.index
    }
}

impl From<&StatusEntry<'_>> for GitStatus {
    fn from(s: &StatusEntry<'_>) -> Self {
        let s = s.status();
        let worktree = if s.is_wt_new() {
            WorktreeStatus::Untracked
        } else if s.is_wt_modified() {
            WorktreeStatus::Modified
        } else if s.is_ignored() {
            WorktreeStatus::Ignored
        } else if s.is_wt_deleted() {
            WorktreeStatus::Deleted
        } else {
            WorktreeStatus::Clean
        };

        let index = if s.is_index_modified() {
            IndexStatus::Modified
        } else if s.is_index_new() {
            IndexStatus::Added
        } else if s.is_index_deleted() {
            IndexStatus::Deleted
        } else {
            IndexStatus::Clean
        };

        Self { worktree, index }
    }
}
