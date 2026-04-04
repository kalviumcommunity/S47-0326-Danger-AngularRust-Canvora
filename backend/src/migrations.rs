use sqlx::{PgPool, migrate::Migrator};
use std::path::Path;

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create migrations table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            installed_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            success BOOLEAN NOT NULL,
            checksum BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Run migrations from the migrations directory
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(pool).await?;

    Ok(())
}

/// Check if migrations are up to date
pub async fn check_migrations(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    let pending = migrator.pending_migrations(pool).await?;
    Ok(pending.is_empty())
}

/// Get migration status
pub async fn get_migration_status(pool: &PgPool) -> Result<Vec<MigrationStatus>, sqlx::Error> {
    let records = sqlx::query!(
        r#"
        SELECT version, description, installed_on, success, execution_time
        FROM _sqlx_migrations
        ORDER BY version ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    let status = records
        .into_iter()
        .map(|record| MigrationStatus {
            version: record.version as u64,
            description: record.description,
            installed_on: record.installed_on,
            success: record.success,
            execution_time_ms: record.execution_time as u64,
        })
        .collect();

    Ok(status)
}

#[derive(Debug, serde::Serialize)]
pub struct MigrationStatus {
    pub version: u64,
    pub description: String,
    pub installed_on: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub execution_time_ms: u64,
}