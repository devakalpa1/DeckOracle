use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::state::AppState;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    timestamp: u64,
    version: String,
    database: String,
}

#[derive(Serialize)]
struct HealthDetails {
    status: String,
    timestamp: u64,
    version: String,
    database: DatabaseHealth,
    uptime: u64,
}

#[derive(Serialize)]
struct DatabaseHealth {
    status: String,
    pool_size: u32,
    idle_connections: usize,
}

/// Simple health check endpoint
pub async fn health() -> impl IntoResponse {
    Json(HealthCheck {
        status: "ok".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: "connected".to_string(),
    })
}

/// Detailed health check with database status
pub async fn health_detailed(State(state): State<AppState>) -> impl IntoResponse {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let pool_options = state.db.options();
    let pool_size = pool_options.get_max_connections();
    let idle_connections = state.db.num_idle();

    Json(HealthDetails {
        status: if db_status == "healthy" { "ok" } else { "degraded" }.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: DatabaseHealth {
            status: db_status.to_string(),
            pool_size,
            idle_connections,
        },
        uptime: 0, // Would need to track server start time for real uptime
    })
}

/// Liveness probe for Kubernetes
pub async fn liveness() -> StatusCode {
    StatusCode::OK
}

/// Readiness probe for Kubernetes - checks database
pub async fn readiness(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
