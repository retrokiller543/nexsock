use crate::daemon_client::get_client;
use crate::state::AppState;
use anyhow::anyhow;
use nexsock_protocol::commands::git::*;
use nexsock_protocol::commands::manage_service::ServiceRef;

/// Get repository status for a service
#[tracing::instrument(skip(state))]
pub async fn get_repo_status(
    state: &AppState,
    service_ref: ServiceRef,
) -> anyhow::Result<RepoStatus> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GetRepoStatusCommand::new(service_ref))
        .await?;

    if res.is_git_status() {
        Ok(res.unwrap_git_status())
    } else {
        Err(anyhow!("Failed to get repository status"))
    }
}

/// List branches for a service
#[tracing::instrument(skip(state))]
pub async fn list_branches(
    state: &AppState,
    service_ref: ServiceRef,
    include_remote: bool,
) -> anyhow::Result<GitListBranchesResponse> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GitListBranchesCommand::new(service_ref, include_remote))
        .await?;

    if res.is_git_branches() {
        Ok(res.unwrap_git_branches())
    } else {
        Err(anyhow!("Failed to list branches"))
    }
}

/// Get git log for a service
#[tracing::instrument(skip(state))]
pub async fn get_log(
    state: &AppState,
    service_ref: ServiceRef,
    max_count: Option<usize>,
    branch: Option<String>,
) -> anyhow::Result<GitLogResponse> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GitLogCommand::new(service_ref, max_count, branch))
        .await?;

    if res.is_git_log() {
        Ok(res.unwrap_git_log())
    } else {
        Err(anyhow!("Failed to get git log"))
    }
}

/// Checkout a branch for a service
#[tracing::instrument(skip(state))]
pub async fn checkout_branch(
    state: &AppState,
    service_ref: ServiceRef,
    branch: String,
    create: bool,
) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let command = if create {
        // For creating branches, we'll use the checkout command with create flag
        // We need to check how the daemon handles branch creation
        CheckoutCommand::new(service_ref, branch)
    } else {
        CheckoutCommand::new(service_ref, branch)
    };

    let res = client.execute_command(command).await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}

/// Checkout a specific commit for a service
#[tracing::instrument(skip(state))]
pub async fn checkout_commit(
    state: &AppState,
    service_ref: ServiceRef,
    commit_hash: String,
) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GitCheckoutCommitCommand::new(service_ref, commit_hash))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}

/// Pull latest changes for a service
#[tracing::instrument(skip(state))]
pub async fn pull(state: &AppState, service_ref: ServiceRef) -> anyhow::Result<()> {
    let mut client = get_client(state).await?;

    let res = client
        .execute_command(GitPullCommand::new(service_ref))
        .await?;

    if res.is_error() {
        Err(anyhow!(res.unwrap_error().message))
    } else {
        Ok(())
    }
}
