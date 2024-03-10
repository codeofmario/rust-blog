use sqlx::{ PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::config::settings::{Settings};

pub async fn init_pool(settings: Settings) -> PgPool {
    let dsn = settings.postgres_dsn.as_str();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(dsn).await.expect("Database connection failed.")
}