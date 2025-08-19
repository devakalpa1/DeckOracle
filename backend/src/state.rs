use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database.url)
            .await?;

        Ok(Self {
            db,
            config: Arc::new(config),
        })
    }
}
