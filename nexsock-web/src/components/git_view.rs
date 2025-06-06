use crate::traits::RenderTemplate;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use nexsock_protocol::commands::git::{GitListBranchesResponse, GitLogResponse, RepoStatus};
use serde::Serialize;

/// Git status component for displaying repository information
#[derive(Debug, Serialize, AsRef, AsMut, Deref, DerefMut, From, Into)]
pub struct GitStatusView(RepoStatus);

impl GitStatusView {
    pub fn new(status: RepoStatus) -> Self {
        Self(status)
    }
}

impl RenderTemplate for GitStatusView {
    const TEMPLATE_NAME: &'static str = "git_status.html";
    const VARIABLE_NAME: &'static str = "git";
}

/// Git branches component for displaying branch information with pagination
#[derive(Debug, Serialize)]
pub struct GitBranchesView {
    pub branches: GitListBranchesResponse,
    pub service_name: String,
    pub show_all: bool,
    pub limit: usize,
}

impl GitBranchesView {
    pub fn new(branches: GitListBranchesResponse, service_name: String) -> Self {
        Self {
            branches,
            service_name,
            show_all: false,
            limit: 10, // Default limit for branches
        }
    }

    pub fn with_show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Get branches to display based on pagination settings
    pub fn displayed_branches(&self) -> &[String] {
        if self.show_all {
            &self.branches.branches
        } else {
            let end = std::cmp::min(self.limit, self.branches.branches.len());
            &self.branches.branches[..end]
        }
    }

    /// Check if there are more branches to show
    pub fn has_more(&self) -> bool {
        !self.show_all && self.branches.branches.len() > self.limit
    }

    /// Get count of remaining branches
    pub fn remaining_count(&self) -> usize {
        if self.show_all {
            0
        } else {
            self.branches.branches.len().saturating_sub(self.limit)
        }
    }
}

impl RenderTemplate for GitBranchesView {
    const TEMPLATE_NAME: &'static str = "git_branches.html";
    const VARIABLE_NAME: &'static str = "branches";
}

/// Git log component for displaying commit history with pagination
#[derive(Debug, Serialize)]
pub struct GitLogView {
    pub log: GitLogResponse,
    pub service_name: String,
    pub show_all: bool,
    pub limit: usize,
}

impl GitLogView {
    pub fn new(log: GitLogResponse, service_name: String) -> Self {
        Self {
            log,
            service_name,
            show_all: false,
            limit: 5, // Default limit for commits
        }
    }

    pub fn with_show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Get commits to display based on pagination settings
    pub fn displayed_commits(&self) -> &[nexsock_protocol::commands::git::GitCommitInfo] {
        if self.show_all {
            &self.log.commits
        } else {
            let end = std::cmp::min(self.limit, self.log.commits.len());
            &self.log.commits[..end]
        }
    }

    /// Check if there are more commits to show
    pub fn has_more(&self) -> bool {
        !self.show_all && self.log.commits.len() > self.limit
    }

    /// Get count of remaining commits
    pub fn remaining_count(&self) -> usize {
        if self.show_all {
            0
        } else {
            self.log.commits.len().saturating_sub(self.limit)
        }
    }
}

impl RenderTemplate for GitLogView {
    const TEMPLATE_NAME: &'static str = "git_log.html";
    const VARIABLE_NAME: &'static str = "log";
}

/// Git section component that combines all git information
#[derive(Debug, Serialize)]
pub struct GitSectionView {
    pub service_name: String,
    pub status: Option<RepoStatus>,
    pub error: Option<String>,
}

impl GitSectionView {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            status: None,
            error: None,
        }
    }

    pub fn with_status(mut self, status: RepoStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }
}

impl RenderTemplate for GitSectionView {
    const TEMPLATE_NAME: &'static str = "git_section.html";
    const VARIABLE_NAME: &'static str = "git";
}
