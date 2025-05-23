//! This crate defines the database migrations for nexsock.
//!
//! It uses `sea-orm-migration` to manage schema changes over time.
//! Each migration is defined in a separate module and registered within
//! the `Migrator` struct.

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_base_service_tables;

/// The main migrator struct that collects all defined migrations.
///
/// This struct implements the `MigratorTrait` from `sea-orm-migration`
/// and provides the `migrations` method to return a list of all
/// migration operations.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// Returns a vector of all migration modules.
    ///
    /// This method is called by the migration runner to discover and apply
    /// the defined migrations in order.
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20220101_000001_create_base_service_tables::Migration,
        )]
    }
}
