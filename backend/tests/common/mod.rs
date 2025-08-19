use deckoracle_backend::config::Config;
use deckoracle_backend::state::AppState;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

/// Create a test database pool with a unique database name
pub async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();
    
    // Connect to postgres to create test database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/postgres".to_string());
    
    // Parse the URL to extract components
    let base_url = database_url.rsplit_once('/').unwrap().0;
    let test_db_name = format!("test_deckoracle_{}", Uuid::new_v4().to_string().replace("-", ""));
    
    // Create a connection to create the test database
    let maintenance_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&format!("{}/postgres", base_url))
        .await
        .expect("Failed to connect to Postgres");
    
    // Create test database
    sqlx::query(&format!("CREATE DATABASE \"{}\"", test_db_name))
        .execute(&maintenance_pool)
        .await
        .expect("Failed to create test database");
    
    // Connect to the test database
    let test_db_url = format!("{}/{}", base_url, test_db_name);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

/// Clean up test database
pub async fn cleanup_test_db(pool: PgPool, db_name: String) {
    // Close all connections to the test database
    pool.close().await;
    
    // Connect to postgres to drop the test database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/postgres".to_string());
    let base_url = database_url.rsplit_once('/').unwrap().0;
    
    let maintenance_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&format!("{}/postgres", base_url))
        .await
        .expect("Failed to connect to Postgres");
    
    sqlx::query(&format!("DROP DATABASE IF EXISTS \"{}\"", db_name))
        .execute(&maintenance_pool)
        .await
        .ok(); // Ignore errors on cleanup
}

/// Create test app state
pub async fn create_test_state() -> Arc<AppState> {
    let pool = setup_test_db().await;
    let config = Config {
        database_url: "test".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 0,
        jwt_secret: "test_secret".to_string(),
        environment: "test".to_string(),
    };
    
    Arc::new(AppState::new(pool, config))
}

/// Test data fixtures
pub mod fixtures {
    use chrono::Utc;
    use uuid::Uuid;
    
    pub fn test_folder_id() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap()
    }
    
    pub fn test_deck_id() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap()
    }
    
    pub fn test_card_id() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap()
    }
}
