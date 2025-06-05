//! This migration creates the initial set of tables for nexsock:
//! - `service_config`: Stores configuration details for services.
//! - `service`: Stores information about manageable services.
//! - `service_dependency`: Stores dependency relationships between services.
//!
//! It also defines necessary indexes for these tables.

use sea_orm_migration::prelude::*;

/// Defines the migration for creating the initial database schema.
///
/// This migration sets up the `ServiceConfig`, `Service`, and `ServiceDependency`
/// tables, along with their respective columns, foreign keys, and indexes.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Applies the migration, creating the tables.
    ///
    /// This method is executed when migrating "up" to this version.
    /// It creates the `ServiceConfig`, `Service`, and `ServiceDependency` tables
    /// in the correct order to satisfy foreign key constraints.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceConfig::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServiceConfig::Filename).string().not_null())
                    .col(
                        ColumnDef::new(ServiceConfig::Format)
                            .string()
                            .not_null()
                            .default("Env")
                            .check(
                                Expr::col(ServiceConfig::Format).is_in(vec!["Env", "Properties"]),
                            ),
                    )
                    .col(ColumnDef::new(ServiceConfig::RunCommand).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Service::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Service::ConfigId).big_integer().null())
                    .col(ColumnDef::new(Service::Name).string().not_null())
                    .col(ColumnDef::new(Service::RepoUrl).string().not_null())
                    .col(ColumnDef::new(Service::Port).big_integer().not_null())
                    .col(ColumnDef::new(Service::RepoPath).string().not_null())
                    .col(
                        ColumnDef::new(Service::Status)
                            .string()
                            .not_null()
                            .default("Stopped")
                            .check(
                                Expr::col(Service::Status)
                                    .is_in(vec!["Starting", "Running", "Stopped", "Failed"]),
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Service::Table, Service::ConfigId)
                            .to(ServiceConfig::Table, ServiceConfig::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("service_name_idx")
                    .table(Service::Table)
                    .col(Service::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("service_config_id_idx")
                    .table(Service::Table)
                    .col(Service::ConfigId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("idx_service_name")
                    .table(Service::Table)
                    .col(Service::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ServiceDependency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceDependency::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceDependency::ServiceId)
                            .big_integer()
                            .not_null()
                            .check(
                                Expr::col(ServiceDependency::ServiceId)
                                    .not_equals(ServiceDependency::DependentServiceId),
                            ),
                    )
                    .col(
                        ColumnDef::new(ServiceDependency::DependentServiceId)
                            .big_integer()
                            .not_null()
                            .check(
                                Expr::col(ServiceDependency::DependentServiceId)
                                    .not_equals(ServiceDependency::ServiceId),
                            ),
                    )
                    .col(
                        ColumnDef::new(ServiceDependency::TunnelEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ServiceDependency::Table, ServiceDependency::ServiceId)
                            .to(Service::Table, Service::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ServiceDependency::Table,
                                ServiceDependency::DependentServiceId,
                            )
                            .to(Service::Table, Service::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("service_dep_idx")
                    .table(ServiceDependency::Table)
                    .col(ServiceDependency::ServiceId)
                    .col(ServiceDependency::DependentServiceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_dependency_service_id")
                    .table(ServiceDependency::Table)
                    .col(ServiceDependency::ServiceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_dependency_dependent_service_id")
                    .table(ServiceDependency::Table)
                    .col(ServiceDependency::DependentServiceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    /// Reverts the migration, dropping the created tables.
    ///
    /// This method is executed when migrating "down" from this version.
    /// It drops the `ServiceDependency`, `Service`, and `ServiceConfig` tables
    /// in the reverse order of their creation to avoid foreign key constraint issues.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceDependency::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Service::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ServiceConfig::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Defines identifiers for the `service_config` table and its columns.
#[derive(Iden)]
enum ServiceConfig {
    /// The name of the `service_config` table.
    Table,
    /// The `id` column, storing the primary key.
    Id,
    /// The `filename` column, storing the name of the configuration file.
    Filename,
    /// The `format` column, storing the format of the configuration file (e.g., "Env", "Properties").
    Format,
    /// The `run_command` column, storing an optional command to run the service.
    RunCommand,
}

/// Defines identifiers for the `service` table and its columns.
#[derive(Iden)]
enum Service {
    /// The name of the `service` table.
    Table,
    /// The `id` column, storing the primary key.
    Id,
    /// The `config_id` column, a foreign key referencing `service_config(id)`.
    ConfigId,
    /// The `name` column, storing the unique name of the service.
    Name,
    /// The `repo_url` column, storing the URL of the service's repository.
    RepoUrl,
    /// The `port` column, storing the port number the service runs on.
    Port,
    /// The `repo_path` column, storing the local filesystem path to the service's repository.
    RepoPath,
    /// The `status` column, storing the current status of the service (e.g., "Starting", "Running", "Stopped", "Failed").
    Status,
}

/// Defines identifiers for the `service_dependency` table and its columns.
#[derive(Iden)]
enum ServiceDependency {
    /// The name of the `service_dependency` table.
    Table,
    /// The `id` column, storing the primary key.
    Id,
    /// The `service_id` column, a foreign key referencing `service(id)`, indicating the service that has a dependency.
    ServiceId,
    /// The `dependent_service_id` column, a foreign key referencing `service(id)`, indicating the service that is the dependency.
    DependentServiceId,
    /// The `tunnel_enabled` column, a boolean indicating if a tunnel is enabled for this dependency.
    TunnelEnabled,
}
