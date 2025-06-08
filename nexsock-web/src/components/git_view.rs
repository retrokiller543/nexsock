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
    pub branches: Vec<String>, // Already paginated list
    pub service_name: String,
    pub has_more: bool,         // Simple boolean
    pub remaining_count: usize, // Simple number
    pub show_all: bool,
}

impl GitBranchesView {
    #[allow(dead_code)]
    pub fn new(branches_response: GitListBranchesResponse, service_name: String) -> Self {
        Self::with_pagination(branches_response, service_name, false, 10)
    }

    #[allow(dead_code)]
    pub fn with_show_all(
        branches_response: GitListBranchesResponse,
        service_name: String,
        show_all: bool,
    ) -> Self {
        Self::with_pagination(branches_response, service_name, show_all, 10)
    }

    #[allow(dead_code)]
    pub fn with_limit(
        branches_response: GitListBranchesResponse,
        service_name: String,
        limit: usize,
    ) -> Self {
        Self::with_pagination(branches_response, service_name, false, limit)
    }

    pub fn with_pagination(
        branches_response: GitListBranchesResponse,
        service_name: String,
        show_all: bool,
        limit: usize,
    ) -> Self {
        let all_branches = branches_response.branches;
        let has_more = !show_all && all_branches.len() > limit;
        let count = all_branches.len();

        let branches = if show_all {
            all_branches.clone()
        } else {
            all_branches.into_iter().take(limit).collect()
        };

        let remaining_count = if show_all {
            0
        } else {
            count.saturating_sub(limit)
        };

        Self {
            branches,
            service_name,
            has_more,
            remaining_count,
            show_all,
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
    pub commits: Vec<nexsock_protocol::commands::git::GitCommitInfo>, // Already paginated
    pub service_name: String,
    pub has_more: bool,
    pub remaining_count: usize,
    pub show_all: bool,
}

impl GitLogView {
    #[allow(dead_code)]
    pub fn new(log_response: GitLogResponse, service_name: String) -> Self {
        Self::with_pagination(log_response, service_name, false, 5)
    }

    #[allow(dead_code)]
    pub fn with_show_all(
        log_response: GitLogResponse,
        service_name: String,
        show_all: bool,
    ) -> Self {
        Self::with_pagination(log_response, service_name, show_all, 5)
    }

    #[allow(dead_code)]
    pub fn with_limit(log_response: GitLogResponse, service_name: String, limit: usize) -> Self {
        Self::with_pagination(log_response, service_name, false, limit)
    }

    pub fn with_pagination(
        log_response: GitLogResponse,
        service_name: String,
        show_all: bool,
        limit: usize,
    ) -> Self {
        let all_commits = log_response.commits;
        let has_more = !show_all && all_commits.len() > limit;
        let count = all_commits.len();

        let commits = if show_all {
            all_commits.clone()
        } else {
            all_commits.into_iter().take(limit).collect()
        };

        let remaining_count = if show_all {
            0
        } else {
            count.saturating_sub(limit)
        };

        Self {
            commits,
            service_name,
            has_more,
            remaining_count,
            show_all,
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
