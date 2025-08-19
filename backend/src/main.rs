mod config;
mod handlers;
mod middleware;
mod models;
mod services;
mod state;
mod utils;

use axum::{
    http::{header, Method},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "deckoracle_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    tracing::info!("Starting DeckOracle backend server...");
    
    // Create application state
    let state = AppState::new(config.clone())
        .await
        .expect("Failed to create application state");

    // Run migrations (skip if already applied)
    if let Err(e) = sqlx::migrate!("./migrations")
        .run(&state.db)
        .await 
    {
        tracing::warn!("Migration warning (may already be applied): {}", e);
    }

    // Build the application routes
    let app = create_app(state, config).await;

    // Get bind address
    let addr: SocketAddr = Config::from_env()
        .expect("Failed to load configuration")
        .get_bind_address()
        .parse()
        .expect("Failed to parse bind address");

    tracing::info!("Server listening on {}", addr);

    // Create the server
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn create_app(state: AppState, config: Config) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors
                .origin
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    // Build the router
    Router::new()
        .nest("/api/v1", api_routes(state))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

fn api_routes(state: AppState) -> Router {
    use axum::routing::get;
    
    Router::new()
        .nest("/auth", handlers::auth::routes())
        .nest("/folders", handlers::folder::routes())
        .nest("/decks", handlers::deck::routes())
        .nest("/cards", handlers::card::routes())
        .nest("/study", handlers::study::routes())
        .nest("/progress", handlers::progress::routes())
        .nest("/import-export", handlers::import_export::routes())
        .nest("/ai", handlers::ai::routes())
        // .nest("/search", handlers::search::routes()) // TODO: Implement search
        // Health check endpoints
        .route("/health", get(handlers::health::health))
        .route("/health/detailed", get(handlers::health::health_detailed))
        .route("/liveness", get(handlers::health::liveness))
        .route("/readiness", get(handlers::health::readiness))
        .with_state(state)
}
