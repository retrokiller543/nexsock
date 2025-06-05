//! This crate defines the database migrations for nexsock.
//!
//! It uses `sea-orm-migration` to manage schema changes over time.
//! Each migration is defined in a separate module and registered within
//! the `Migrator` struct.

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_base_service_tables;
mod m20250605_000002_add_git_columns;

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
    /// Returns the list of database migrations to be applied, in sequential order.
    ///
    /// The migrations are boxed and ordered to ensure correct application by the migration runner.
    ///
    /// # Returns
    /// A vector of boxed migration instances implementing `MigrationTrait`.
    ///
    /// # Examples
    ///
    /// ```
    /// let migrations = Migrator::migrations();
    /// assert!(!migrations.is_empty());
    /// ```
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_base_service_tables::Migration),
            Box::new(m20250605_000002_add_git_columns::Migration),
        ]
    }
}
