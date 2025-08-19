use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::UserId,
    models::{Card, CreateCardDto, UpdateCardDto},
    services::card::CardService,
    state::AppState,
    utils::{AppError, Result},
};

#[derive(Deserialize)]
struct CardsQuery {
    deck_id: Uuid,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_cards).post(create_card))
        .route("/bulk", post(bulk_create_cards))
        .route("/:id", get(get_card).patch(update_card).delete(delete_card))
}

async fn list_cards(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<CardsQuery>,
) -> Result<Json<Vec<Card>>> {
    let cards = CardService::list_deck_cards(&state.db, query.deck_id, user_id).await?;
    Ok(Json(cards))
}

async fn create_card(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<CardsQuery>,
    Json(dto): Json<CreateCardDto>,
) -> Result<(StatusCode, Json<Card>)> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let card = CardService::create_card(&state.db, query.deck_id, user_id, dto).await?;
    Ok((StatusCode::CREATED, Json(card)))
}

async fn get_card(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<Card>> {
    let card = CardService::get_card(&state.db, id, user_id).await?;
    Ok(Json(card))
}

async fn update_card(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateCardDto>,
) -> Result<Json<Card>> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let card = CardService::update_card(&state.db, id, user_id, dto).await?;
    Ok(Json(card))
}

async fn delete_card(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    CardService::delete_card(&state.db, id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn bulk_create_cards(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<CardsQuery>,
    Json(cards): Json<Vec<CreateCardDto>>,
) -> Result<(StatusCode, Json<Vec<Card>>)> {
    // Validate all cards
    for card in &cards {
        card.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
    }
    
    let created_cards = CardService::bulk_create_cards(&state.db, query.deck_id, user_id, cards).await?;
    Ok((StatusCode::CREATED, Json(created_cards)))
}
