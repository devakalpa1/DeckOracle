// Stub implementations for missing endpoints to prevent 404 errors
// These will be properly implemented in Phase 3

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    middleware::auth::UserId,
    state::AppState,
    utils::Result,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stats", get(get_user_stats))
        .route("/achievements", get(get_user_achievements))
}

/// Stub implementation for user stats
/// Returns empty but valid response to prevent frontend errors
async fn get_user_stats(
    _state: State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Value>> {
    Ok(Json(json!({
        "user_id": user_id,
        "total_cards_studied": 0,
        "total_study_time_seconds": 0,
        "current_streak_days": 0,
        "longest_streak_days": 0,
        "total_points": 0,
        "level": 1,
        "message": "Stats feature coming soon",
        "feature_enabled": false
    })))
}

/// Stub implementation for achievements
/// Returns empty array to prevent frontend errors
async fn get_user_achievements(
    _state: State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<Value>>> {
    // Return empty array for now
    // Frontend will show "Coming Soon" badge based on feature flag
    Ok(Json(vec![]))
}

// Additional stub for progress endpoints
pub mod progress_stub {
    use super::*;
    
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/overview", get(get_progress_overview))
            .route("/decks", get(get_deck_progress))
            .route("/learning-curve", get(get_learning_curve))
            .route("/streaks", get(get_study_streaks))
            .route("/weekly", get(get_weekly_progress))
    }
    
    async fn get_progress_overview(
        _state: State<AppState>,
        UserId(_user_id): UserId,
    ) -> Result<Json<Value>> {
        Ok(Json(json!({
            "cards_studied_today": 0,
            "study_time_today": 0,
            "accuracy_today": 0.0,
            "feature_enabled": false
        })))
    }
    
    async fn get_deck_progress(
        _state: State<AppState>,
        UserId(_user_id): UserId,
    ) -> Result<Json<Vec<Value>>> {
        Ok(Json(vec![]))
    }
    
    async fn get_learning_curve(
        _state: State<AppState>,
        UserId(_user_id): UserId,
    ) -> Result<Json<Vec<Value>>> {
        Ok(Json(vec![]))
    }
    
    async fn get_study_streaks(
        _state: State<AppState>,
        UserId(_user_id): UserId,
    ) -> Result<Json<Value>> {
        Ok(Json(json!({
            "current_streak": 0,
            "longest_streak": 0,
            "last_study_date": null
        })))
    }
    
    async fn get_weekly_progress(
        _state: State<AppState>,
        UserId(_user_id): UserId,
    ) -> Result<Json<Vec<Value>>> {
        Ok(Json(vec![]))
    }
}
