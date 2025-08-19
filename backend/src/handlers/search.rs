use axum::{
    extract::{Query, State},
    Json, Router,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{Card, Deck, DeckWithStats},
    services::search::SearchService,
    state::AppState,
    utils::{PaginatedResponse, PaginationParams, Result},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(search_all))
        .route("/decks", get(search_decks))
        .route("/cards", get(search_cards))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[derive(Serialize)]
struct SearchResults {
    decks: Vec<DeckWithStats>,
    cards: Vec<CardSearchResult>,
}

#[derive(Serialize)]
pub struct CardSearchResult {
    #[serde(flatten)]
    pub card: Card,
    pub deck_name: String,
    pub deck_id: Uuid,
}

async fn search_all(
    State(state): State<AppState>,
    Query(mut query): Query<SearchQuery>,
) -> Result<Json<SearchResults>> {
    // TODO: Get user_id from auth middleware
    let user_id = Uuid::new_v4(); // Placeholder
    
    // Validate and clean search query
    let search_term = query.q.trim();
    if search_term.is_empty() {
        return Ok(Json(SearchResults {
            decks: vec![],
            cards: vec![],
        }));
    }
    
    query.pagination.validate();
    
    // Search both decks and cards (limited results for overview)
    let decks = SearchService::search_decks(
        &state.db,
        user_id,
        search_term,
        5, // Limit to 5 decks in combined search
    ).await?;
    
    let cards = SearchService::search_cards(
        &state.db,
        user_id,
        search_term,
        10, // Limit to 10 cards in combined search
    ).await?;
    
    Ok(Json(SearchResults { decks, cards }))
}

async fn search_decks(
    State(state): State<AppState>,
    Query(mut query): Query<SearchQuery>,
) -> Result<Json<PaginatedResponse<DeckWithStats>>> {
    // TODO: Get user_id from auth middleware
    let user_id = Uuid::new_v4(); // Placeholder
    
    let search_term = query.q.trim();
    if search_term.is_empty() {
        return Ok(Json(PaginatedResponse::new(vec![], &query.pagination, Some(0))));
    }
    
    query.pagination.validate();
    
    let decks = SearchService::search_decks_paginated(
        &state.db,
        user_id,
        search_term,
        &query.pagination,
    ).await?;
    
    Ok(Json(decks))
}

async fn search_cards(
    State(state): State<AppState>,
    Query(mut query): Query<SearchQuery>,
) -> Result<Json<PaginatedResponse<CardSearchResult>>> {
    // TODO: Get user_id from auth middleware
    let user_id = Uuid::new_v4(); // Placeholder
    
    let search_term = query.q.trim();
    if search_term.is_empty() {
        return Ok(Json(PaginatedResponse::new(vec![], &query.pagination, Some(0))));
    }
    
    query.pagination.validate();
    
    let cards = SearchService::search_cards_paginated(
        &state.db,
        user_id,
        search_term,
        &query.pagination,
    ).await?;
    
    Ok(Json(cards))
}
