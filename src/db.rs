use sqlx::PgPool;
use anyhow::Result;
use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn connect_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    MIGRATOR.run(&pool).await.expect("Failed to run migrations");
    
    Ok(pool)
}