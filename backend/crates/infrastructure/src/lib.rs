use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("database connection failed: {0}")]
    Connection(sqlx::Error),
}

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, InfrastructureError> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.acquire_timeout)
        .connect(&config.url)
        .await
        .map_err(InfrastructureError::Connection)
}

pub struct RuntimeStatus {
    pub database_ready: bool,
    pub storage_ready: bool,
}

impl RuntimeStatus {
    pub fn ready() -> Self {
        Self {
            database_ready: true,
            storage_ready: true,
        }
    }
}
