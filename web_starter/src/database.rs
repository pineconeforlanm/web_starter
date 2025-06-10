use super::config;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let db_cfg = &config::get().database();
    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_cfg.user(),
        db_cfg.password(),
        db_cfg.host(),
        db_cfg.port(),
        db_cfg.database()
    ));

    options
        .min_connections(db_cfg.min_connections())
        .max_connections(db_cfg.max_connections())
        .connect_timeout(Duration::from_secs(db_cfg.connect_timeout()))
        .acquire_timeout(Duration::from_secs(db_cfg.acquire_timeout()))
        .idle_timeout(Duration::from_secs(db_cfg.idle_timeout()))
        .max_lifetime(Duration::from_secs(db_cfg.max_lifetime()))
        .sqlx_logging(db_cfg.sqlx_logging())
        .set_schema_search_path(db_cfg.schema());

    let db = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("Database connected successfully");

    log_database_version(&db).await?;

    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version_result = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?;

    tracing::info!(
        "Database version: {}",
        version_result.try_get_by_index::<String>(0)?
    );

    Ok(())
}
