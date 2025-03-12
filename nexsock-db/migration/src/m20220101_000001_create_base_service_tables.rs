use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create service_config table first since it's referenced by service
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

        // Create service table with reference to service_config
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

        // Create indexes for service table separately
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

        // Create unique index on service name
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

        // Create service_dependency table
        manager
            .create_table(
                Table::create()
                    .table(ServiceDependency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceDependency::Id)
                            .big_integer() // Changed to big_integer for consistency
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceDependency::ServiceId)
                            .big_integer()
                            .not_null()
                            .check(
                                ServiceDependency::ServiceId
                                    .into_column_ref()
                                    .not_equals(ServiceDependency::DependentServiceId),
                            ),
                    )
                    .col(
                        ColumnDef::new(ServiceDependency::DependentServiceId)
                            .big_integer()
                            .not_null()
                            .check(
                                ServiceDependency::DependentServiceId
                                    .into_column_ref()
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

        // Create check constraint for service_dependency separately
        /*manager.exec_stmt(
            Query::raw(
                &format!(
                    "ALTER TABLE {} ADD CONSTRAINT check_service_dependency CHECK ({} != {})",
                    ServiceDependency::Table,
                    ServiceDependency::ServiceId,
                    ServiceDependency::DependentServiceId
                )
            )
        )
            .await?;*/

        // Create unique composite index on service_dependency
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

        // Add additional performance indexes
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation
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

// Define table names and columns using Sea-ORM's naming conventions
#[derive(Iden)]
enum ServiceConfig {
    Table,
    Id,
    Filename,
    Format,
    RunCommand,
}

#[derive(Iden)]
enum Service {
    Table,
    Id,
    ConfigId,
    Name,
    RepoUrl,
    Port,
    RepoPath,
    Status,
}

#[derive(Iden)]
enum ServiceDependency {
    Table,
    Id,
    ServiceId,
    DependentServiceId,
    TunnelEnabled,
}
