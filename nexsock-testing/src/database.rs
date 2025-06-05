use anyhow::Result;
use nexsock_db::prelude::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn setup_test_db() -> Result<DatabaseConnection> {
    let db_url = "sqlite::memory:";

    let mut opt = ConnectOptions::new(db_url.to_string());
    opt.connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(5 * 60))
        .sqlx_logging(false)
        .max_connections(1)
        .min_connections(1);

    let conn = Database::connect(opt).await?;
    Migrator::up(&conn, None).await?;

    Ok(conn)
}

pub async fn setup_test_db_with_url(db_url: &str) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(db_url.to_string());
    opt.connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(5 * 60))
        .sqlx_logging(false);

    let conn = Database::connect(opt).await?;
    Migrator::up(&conn, None).await?;

    Ok(conn)
}

pub async fn reset_database(conn: &DatabaseConnection) -> Result<()> {
    Migrator::down(conn, None).await?;
    Migrator::up(conn, None).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexsock_db::prelude::ServiceRepository;

    #[tokio::test]
    async fn test_database_setup() {
        let db = setup_test_db()
            .await
            .expect("Failed to setup test database");

        // Verify we can create a repository with the database
        let _repo = ServiceRepository::new(&db);

        // The fact that this doesn't panic means the database is properly set up
    }

    // TODO: SQLite doesn't support multiple alter options for table drops/recreates
    // #[tokio::test]
    // async fn test_database_reset() {
    //     let db = setup_test_db().await.expect("Failed to setup test database");
    //
    //     // Reset should work without errors
    //     reset_database(&db).await.expect("Failed to reset database");
    //
    //     // Should still be able to create a repository after reset
    //     let _repo = ServiceRepository::new(&db);
    // }
}
