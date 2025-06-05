//! This migration adds Git-related columns to the service table to support
//! Git repository management including branch tracking, commit hashes, and
//! authentication configuration.

use sea_orm_migration::prelude::*;

macro_rules! add_column {
    ($manager:expr, $table:expr, $column:expr) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table)
                    .add_column($column)
                    .to_owned(),
            )
            .await?;
    };
}

/// Defines the migration for adding Git-related columns to the service table.
///
/// This migration adds columns to track Git repository state including:
/// - Current branch name
/// - Current commit hash  
/// - Authentication type used for Git operations
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Applies the migration, adding Git columns to the service table.
    ///
    /// This method adds the following columns to the `service` table:
    /// - `git_branch`: Optional text field storing the current Git branch name
    /// - `git_commit_hash`: Optional text field storing the current commit SHA
    /// - `git_auth_type`: Optional text field storing the authentication method
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        add_column!(manager, Service::Table, ColumnDef::new(Service::GitBranch).string().null());
        add_column!(manager, Service::Table, ColumnDef::new(Service::GitCommitHash).string().null());
        add_column!(manager, Service::Table, 
            ColumnDef::new(Service::GitAuthType)
                .string()
                .null()
                .check(
                    Expr::col(Service::GitAuthType).is_in(vec![
                        "none",
                        "ssh_agent", 
                        "ssh_key",
                        "token",
                        "user_pass"
                    ])
                )
        );

        // Add index on git_branch for efficient branch-based queries
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_service_git_branch")
                    .table(Service::Table)
                    .col(Service::GitBranch)
                    .to_owned(),
            )
            .await?;

        // Add index on git_commit_hash for efficient commit-based queries
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_service_git_commit_hash")
                    .table(Service::Table)
                    .col(Service::GitCommitHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    /// Reverts the migration, removing Git columns from the service table.
    ///
    /// This method drops the Git-related columns and their associated indexes.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_service_git_commit_hash")
                    .table(Service::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_service_git_branch")
                    .table(Service::Table)
                    .to_owned(),
            )
            .await?;

        // Drop columns
        manager
            .alter_table(
                Table::alter()
                    .table(Service::Table)
                    .drop_column(Service::GitAuthType)
                    .drop_column(Service::GitCommitHash)
                    .drop_column(Service::GitBranch)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Defines identifiers for the `service` table and its Git-related columns.
#[derive(Iden)]
enum Service {
    /// The name of the `service` table.
    Table,
    /// The `git_branch` column, storing the current Git branch name.
    GitBranch,
    /// The `git_commit_hash` column, storing the current commit SHA.
    GitCommitHash,
    /// The `git_auth_type` column, storing the authentication method.
    GitAuthType,
}