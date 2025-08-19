use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    handlers::search::CardSearchResult,
    models::{Card, Deck, DeckWithStats},
    utils::{PaginatedResponse, PaginationParams, Result},
};

pub struct SearchService;

impl SearchService {
    /// Search decks by name or description
    pub async fn search_decks(
        db: &PgPool,
        user_id: Uuid,
        search_term: &str,
        limit: i64,
    ) -> Result<Vec<DeckWithStats>> {
        let search_pattern = format!("%{}%", search_term);
        
        let decks = sqlx::query!(
            r#"
            SELECT 
                d.id,
                d.folder_id,
                d.owner_id as user_id,
                d.title as name,
                d.description,
                d.is_public,
                d.created_at,
                d.updated_at,
                COUNT(c.id) as "card_count!",
                MAX(ss.started_at) as last_studied
            FROM decks d
            LEFT JOIN cards c ON c.deck_id = d.id
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = $1
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(d.title) LIKE LOWER($2) OR LOWER(d.description) LIKE LOWER($2))
            GROUP BY d.id
            ORDER BY 
                CASE WHEN LOWER(d.title) LIKE LOWER($2) THEN 0 ELSE 1 END,
                d.title
            LIMIT $3
            "#,
            user_id,
            search_pattern,
            limit
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|r| DeckWithStats {
            deck: Deck {
                id: r.id,
                folder_id: r.folder_id,
                user_id: r.user_id,
                name: r.name,
                description: r.description,
                is_public: r.is_public,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            card_count: r.card_count,
            last_studied: r.last_studied,
        })
        .collect();

        Ok(decks)
    }

    /// Search decks with pagination
    pub async fn search_decks_paginated(
        db: &PgPool,
        user_id: Uuid,
        search_term: &str,
        params: &PaginationParams,
    ) -> Result<PaginatedResponse<DeckWithStats>> {
        let search_pattern = format!("%{}%", search_term);
        let offset = params.offset() as i64;
        let limit = params.limit_plus_one() as i64;
        
        let decks = sqlx::query!(
            r#"
            SELECT 
                d.id,
                d.folder_id,
                d.owner_id as user_id,
                d.title as name,
                d.description,
                d.is_public,
                d.created_at,
                d.updated_at,
                COUNT(c.id) as "card_count!",
                MAX(ss.started_at) as last_studied
            FROM decks d
            LEFT JOIN cards c ON c.deck_id = d.id
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = $1
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(d.title) LIKE LOWER($2) OR LOWER(d.description) LIKE LOWER($2))
            GROUP BY d.id
            ORDER BY 
                CASE WHEN LOWER(d.title) LIKE LOWER($2) THEN 0 ELSE 1 END,
                d.title
            LIMIT $3 OFFSET $4
            "#,
            user_id,
            search_pattern,
            limit,
            offset
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|r| DeckWithStats {
            deck: Deck {
                id: r.id,
                folder_id: r.folder_id,
                user_id: r.user_id,
                name: r.name,
                description: r.description,
                is_public: r.is_public,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            card_count: r.card_count,
            last_studied: r.last_studied,
        })
        .collect();

        // Get total count for pagination
        let total = sqlx::query!(
            r#"
            SELECT COUNT(DISTINCT d.id) as "count!"
            FROM decks d
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(d.title) LIKE LOWER($2) OR LOWER(d.description) LIKE LOWER($2))
            "#,
            user_id,
            search_pattern
        )
        .fetch_one(db)
        .await?
        .count as u32;

        Ok(PaginatedResponse::new(decks, params, Some(total)))
    }

    /// Search cards by front or back content
    pub async fn search_cards(
        db: &PgPool,
        user_id: Uuid,
        search_term: &str,
        limit: i64,
    ) -> Result<Vec<CardSearchResult>> {
        let search_pattern = format!("%{}%", search_term);
        
        let cards = sqlx::query!(
            r#"
            SELECT 
                c.id,
                c.deck_id,
                c.front,
                c.back,
                c.position,
                c.created_at,
                c.updated_at,
                d.title as deck_name
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(c.front) LIKE LOWER($2) OR LOWER(c.back) LIKE LOWER($2))
            ORDER BY 
                CASE WHEN LOWER(c.front) LIKE LOWER($2) THEN 0 ELSE 1 END,
                c.position
            LIMIT $3
            "#,
            user_id,
            search_pattern,
            limit
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|r| CardSearchResult {
            card: Card {
                id: r.id,
                deck_id: r.deck_id,
                front: r.front,
                back: r.back,
                position: r.position,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            deck_name: r.deck_name,
            deck_id: r.deck_id,
        })
        .collect();

        Ok(cards)
    }

    /// Search cards with pagination
    pub async fn search_cards_paginated(
        db: &PgPool,
        user_id: Uuid,
        search_term: &str,
        params: &PaginationParams,
    ) -> Result<PaginatedResponse<CardSearchResult>> {
        let search_pattern = format!("%{}%", search_term);
        let offset = params.offset() as i64;
        let limit = params.limit_plus_one() as i64;
        
        let cards = sqlx::query!(
            r#"
            SELECT 
                c.id,
                c.deck_id,
                c.front,
                c.back,
                c.position,
                c.created_at,
                c.updated_at,
                d.title as deck_name
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(c.front) LIKE LOWER($2) OR LOWER(c.back) LIKE LOWER($2))
            ORDER BY 
                CASE WHEN LOWER(c.front) LIKE LOWER($2) THEN 0 ELSE 1 END,
                c.position
            LIMIT $3 OFFSET $4
            "#,
            user_id,
            search_pattern,
            limit,
            offset
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|r| CardSearchResult {
            card: Card {
                id: r.id,
                deck_id: r.deck_id,
                front: r.front,
                back: r.back,
                position: r.position,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            deck_name: r.deck_name,
            deck_id: r.deck_id,
        })
        .collect();

        // Get total count for pagination
        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE (d.owner_id = $1 OR d.is_public = true)
              AND (LOWER(c.front) LIKE LOWER($2) OR LOWER(c.back) LIKE LOWER($2))
            "#,
            user_id,
            search_pattern
        )
        .fetch_one(db)
        .await?
        .count as u32;

        Ok(PaginatedResponse::new(cards, params, Some(total)))
    }
}
