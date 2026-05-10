// Миграции применяются при запуске проекта

use sqlx::PgPool;
use anyhow::{Result, Context};  // For .context()

// (!) Работает только с ""./migrations" - Относительно Cargo.toml
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn run(pool: &PgPool) -> Result<()> {
    MIGRATOR
        .run(pool)
        .await
        .context("Failed to run database migrations")  // ✅ Adds context + converts error
}


// Код с курсов 

// use sqlx::{PgPool};  // ✅ Explicit import
// static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("migrations");
// pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
//     MIGRATOR.run(pool).await
// } 
