use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{Card, CreateCardDto, UpdateCardDto},
    utils::{AppError, Result},
};

pub struct CardService;

impl CardService {
    pub async fn list_deck_cards(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<Card>> {
        // First verify deck access
        let deck_access = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM decks
                WHERE id = $1 AND owner_id = $2
            ) as "exists!"
            "#,
            deck_id,
            user_id
        )
        .fetch_one(db)
        .await?
        .exists;

        if !deck_access {
            return Err(AppError::NotFound("Resource not found".to_string()));
        }

        let cards = sqlx::query_as!(
            Card,
            r#"
            SELECT id, deck_id, front, back, position, created_at, updated_at
            FROM cards
            WHERE deck_id = $1
            ORDER BY position
            "#,
            deck_id
        )
        .fetch_all(db)
        .await?;

        Ok(cards)
    }

    pub async fn create_card(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
        dto: CreateCardDto,
    ) -> Result<Card> {
        // Verify deck ownership
        let deck_owner = sqlx::query!(
            r#"
            SELECT owner_id as user_id
            FROM decks
            WHERE id = $1
            "#,
            deck_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        if deck_owner.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        // Get position if not provided
        let position = match dto.position {
            Some(pos) => pos,
            None => {
                let max_position = sqlx::query!(
                    r#"
                    SELECT COALESCE(MAX(position), -1) as "max_position!"
                    FROM cards
                    WHERE deck_id = $1
                    "#,
                    deck_id
                )
                .fetch_one(db)
                .await?
                .max_position;

                max_position + 1
            }
        };

        let card = sqlx::query_as!(
            Card,
            r#"
            INSERT INTO cards (deck_id, front, back, position)
            VALUES ($1, $2, $3, $4)
            RETURNING id, deck_id, front, back, position, created_at, updated_at
            "#,
            deck_id,
            dto.front,
            dto.back,
            position
        )
        .fetch_one(db)
        .await?;

        Ok(card)
    }

    pub async fn get_card(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Card> {
        let card = sqlx::query_as!(
            Card,
            r#"
            SELECT c.id, c.deck_id, c.front, c.back, c.position, c.created_at, c.updated_at
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE c.id = $1 AND d.owner_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        Ok(card)
    }

    pub async fn update_card(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
        dto: UpdateCardDto,
    ) -> Result<Card> {
        // Verify ownership through deck
        let deck_owner = sqlx::query!(
            r#"
            SELECT d.owner_id as user_id
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE c.id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        if deck_owner.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        let card = sqlx::query_as!(
            Card,
            r#"
            UPDATE cards
            SET 
                front = COALESCE($2, front),
                back = COALESCE($3, back),
                position = COALESCE($4, position)
            WHERE id = $1
            RETURNING id, deck_id, front, back, position, created_at, updated_at
            "#,
            id,
            dto.front,
            dto.back,
            dto.position
        )
        .fetch_one(db)
        .await?;

        Ok(card)
    }

    pub async fn delete_card(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<()> {
        // Verify ownership through deck
        let deck_owner = sqlx::query!(
            r#"
            SELECT d.owner_id as user_id
            FROM cards c
            JOIN decks d ON d.id = c.deck_id
            WHERE c.id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        if deck_owner.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        sqlx::query!(
            r#"
            DELETE FROM cards
            WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn bulk_create_cards(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
        cards: Vec<CreateCardDto>,
    ) -> Result<Vec<Card>> {
        // Verify deck ownership
        let deck_owner = sqlx::query!(
            r#"
            SELECT owner_id as user_id
            FROM decks
            WHERE id = $1
            "#,
            deck_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        if deck_owner.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        // Get current max position
        let max_position = sqlx::query!(
            r#"
            SELECT COALESCE(MAX(position), -1) as "max_position!"
            FROM cards
            WHERE deck_id = $1
            "#,
            deck_id
        )
        .fetch_one(db)
        .await?
        .max_position;

        let mut created_cards = Vec::new();
        let mut position = max_position + 1;

        // Create cards in a transaction
        let mut tx = db.begin().await?;

        for card_dto in cards {
            let card = sqlx::query_as!(
                Card,
                r#"
                INSERT INTO cards (deck_id, front, back, position)
                VALUES ($1, $2, $3, $4)
                RETURNING id, deck_id, front, back, position, created_at, updated_at
                "#,
                deck_id,
                card_dto.front,
                card_dto.back,
                card_dto.position.unwrap_or(position)
            )
            .fetch_one(&mut *tx)
            .await?;

            created_cards.push(card);
            position += 1;
        }

        tx.commit().await?;

        Ok(created_cards)
    }
}
