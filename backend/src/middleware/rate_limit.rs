use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Duration, Utc};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::RwLock;

use crate::utils::AppError;

/// Rate limit configuration
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_seconds: i64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,  // 100 requests
            window_seconds: 60,  // per minute
        }
    }
}

/// Store for rate limit tracking
#[derive(Clone)]
pub struct RateLimitStore {
    requests: Arc<RwLock<HashMap<String, Vec<DateTime<Utc>>>>>,
    config: RateLimitConfig,
}

impl RateLimitStore {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Check if a client has exceeded the rate limit
    async fn check_rate_limit(&self, client_id: &str) -> bool {
        let mut requests = self.requests.write().await;
        let now = Utc::now();
        let window_start = now - Duration::seconds(self.config.window_seconds);

        // Get or create request history for this client
        let client_requests = requests.entry(client_id.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside the window
        client_requests.retain(|timestamp| *timestamp > window_start);

        // Check if limit exceeded
        if client_requests.len() >= self.config.max_requests as usize {
            return false; // Rate limit exceeded
        }

        // Add current request
        client_requests.push(now);
        true
    }

    /// Clean up old entries periodically (should be called by a background task)
    pub async fn cleanup(&self) {
        let mut requests = self.requests.write().await;
        let now = Utc::now();
        let window_start = now - Duration::seconds(self.config.window_seconds * 2);

        // Remove entries that have no recent requests
        requests.retain(|_, timestamps| {
            timestamps.retain(|timestamp| *timestamp > window_start);
            !timestamps.is_empty()
        });
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(store): State<RateLimitStore>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Use IP address as client identifier
    let client_id = addr.ip().to_string();

    // Check rate limit
    if !store.check_rate_limit(&client_id).await {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests. Please try again later.",
        ).into_response());
    }

    // Continue with the request
    Ok(next.run(request).await)
}

/// Create a rate limit layer for specific endpoints (like login)
pub fn create_auth_rate_limiter() -> RateLimitStore {
    RateLimitStore::new(RateLimitConfig {
        max_requests: 5,     // 5 attempts
        window_seconds: 900, // per 15 minutes
    })
}

/// Create a general rate limiter for API endpoints
pub fn create_api_rate_limiter() -> RateLimitStore {
    RateLimitStore::new(RateLimitConfig {
        max_requests: 1000,  // 1000 requests
        window_seconds: 60,  // per minute
    })
}
