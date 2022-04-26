use sqlx::PgPool;
use anyhow::Result;
use sqlx::migrate::Migrator;
use crate::app_config::get_config;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn connect_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    MIGRATOR.run(&pool).await.expect("Failed to run migrations");
    
    Ok(pool)
}

pub async fn get_db() -> Result<PgPool> {
    let config = get_config();
    let pool = PgPool::connect(&config.database_url).await?;
    Ok(pool)
}