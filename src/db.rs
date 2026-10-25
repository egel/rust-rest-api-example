//! Database connection pool setup.

use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Maximum number of connections kept in the pool.
const MAX_CONNECTIONS: u32 = 10;

/// How long acquiring a pooled connection may block before giving up.
const ACQUIRE_TIMEOUT_SECS: u64 = 5;

/// Initializes the connection pool for the given `database_url`.
pub async fn connect(database_url: String) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_secs(ACQUIRE_TIMEOUT_SECS))
        .connect(&database_url)
        .await
}
