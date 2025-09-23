//! This migration adds support for default environment variables and service aliases.
//! 
//! Creates two new tables:
//! - `service_env_var`: Stores default environment variables for services
//! - `service_alias`: Stores alternative names/aliases for services

use sea_orm_migration::prelude::*;

/// Migration to add environment variables and aliases support
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Creates the service_env_var and service_alias tables
    ///
    /// This migration adds support for:
    /// - Default environment variables per service (key-value pairs)
    /// - Service aliases (alternative names for services)
    ///
    /// Both tables have foreign key relationships to the service table
    /// with CASCADE delete to maintain referential integrity.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create service_env_var table
        manager
            .create_table(
                Table::create()
                    .table(ServiceEnvVar::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceEnvVar::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceEnvVar::ServiceId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceEnvVar::Key)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceEnvVar::Value)
                            .text()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ServiceEnvVar::Table, ServiceEnvVar::ServiceId)
                            .to(Service::Table, Service::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index for service_id + key combination
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("unique_service_env_var")
                    .table(ServiceEnvVar::Table)
                    .col(ServiceEnvVar::ServiceId)
                    .col(ServiceEnvVar::Key)
                    .to_owned(),
            )
            .await?;

        // Create index for faster lookups by service_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_service_env_var_service_id")
                    .table(ServiceEnvVar::Table)
                    .col(ServiceEnvVar::ServiceId)
                    .to_owned(),
            )
            .await?;

        // Create service_alias table
        manager
            .create_table(
                Table::create()
                    .table(ServiceAlias::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceAlias::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceAlias::ServiceId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceAlias::Alias)
                            .string_len(255)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ServiceAlias::Table, ServiceAlias::ServiceId)
                            .to(Service::Table, Service::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for faster lookups by service_id
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_service_alias_service_id")
                    .table(ServiceAlias::Table)
                    .col(ServiceAlias::ServiceId)
                    .to_owned(),
            )
            .await?;

        // Create unique index for alias lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .unique()
                    .name("idx_service_alias_alias")
                    .table(ServiceAlias::Table)
                    .col(ServiceAlias::Alias)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    /// Drops the service_env_var and service_alias tables
    ///
    /// Reverts this migration by removing the tables in reverse order
    /// to avoid foreign key constraint violations.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceAlias::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(ServiceEnvVar::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Identifiers for the service_env_var table and its columns
#[derive(Iden)]
enum ServiceEnvVar {
    /// The service_env_var table name
    Table,
    /// Primary key
    Id,
    /// Foreign key to service table
    ServiceId,
    /// Environment variable key/name
    Key,
    /// Environment variable value
    Value,
}

/// Identifiers for the service_alias table and its columns
#[derive(Iden)]
enum ServiceAlias {
    /// The service_alias table name
    Table,
    /// Primary key
    Id,
    /// Foreign key to service table
    ServiceId,
    /// Alias name (must be unique across all services)
    Alias,
}

/// Identifiers for the existing service table (for foreign key references)
#[derive(Iden)]
enum Service {
    /// The service table name
    Table,
    /// Primary key of service table
    Id,
}