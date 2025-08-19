use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::UserId,
    models::{CreateDeckDto, Deck, DeckWithStats, UpdateDeckDto},
    services::deck::DeckService,
    state::AppState,
    utils::{AppError, Result},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_decks).post(create_deck))
        .route("/:id", get(get_deck).patch(update_deck).delete(delete_deck))
        .route("/:id/stats", get(get_deck_with_stats))
        .route("/:id/csv", post(import_csv).get(export_csv))
}

async fn list_decks(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<DeckWithStats>>> {
    let decks = DeckService::list_user_decks(&state.db, user_id).await?;
    Ok(Json(decks))
}

async fn create_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Json(dto): Json<CreateDeckDto>,
) -> Result<(StatusCode, Json<Deck>)> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let deck = DeckService::create_deck(&state.db, user_id, dto).await?;
    Ok((StatusCode::CREATED, Json(deck)))
}

async fn get_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<Deck>> {
    let deck = DeckService::get_deck(&state.db, id, user_id).await?;
    Ok(Json(deck))
}

async fn get_deck_with_stats(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<DeckWithStats>> {
    let deck_stats = DeckService::get_deck_with_stats(&state.db, id, user_id).await?;
    Ok(Json(deck_stats))
}

async fn update_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateDeckDto>,
) -> Result<Json<Deck>> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let deck = DeckService::update_deck(&state.db, id, user_id, dto).await?;
    Ok(Json(deck))
}

async fn delete_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    DeckService::delete_deck(&state.db, id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn import_csv(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
    body: String,
) -> Result<Json<serde_json::Value>> {
    let cards = DeckService::import_csv(&state.db, id, user_id, body).await?;
    
    Ok(Json(serde_json::json!({
        "message": "CSV imported successfully",
        "cards_created": cards.len(),
        "cards": cards
    })))
}

async fn export_csv(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Response> {
    let csv_content = DeckService::export_csv(&state.db, id, user_id).await?;
    
    // Get deck name for filename
    let deck = DeckService::get_deck(&state.db, id, user_id).await?;
    let filename = format!("{}.csv", deck.name.replace(' ', "_"));
    
    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                &format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        csv_content,
    )
        .into_response())
}
