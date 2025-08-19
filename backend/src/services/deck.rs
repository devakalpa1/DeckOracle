use csv::{Reader, Writer};
use sqlx::PgPool;
use std::io::Cursor;
use uuid::Uuid;

use crate::{
    models::{Card, CreateDeckDto, CsvCard, Deck, DeckWithStats, UpdateDeckDto},
    utils::{AppError, Result},
};

pub struct DeckService;

impl DeckService {
    pub async fn list_user_decks(db: &PgPool, user_id: Uuid) -> Result<Vec<DeckWithStats>> {
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
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = d.owner_id
            WHERE d.owner_id = $1
            GROUP BY d.id
            ORDER BY d.title
            "#,
            user_id
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

    pub async fn create_deck(
        db: &PgPool,
        user_id: Uuid,
        dto: CreateDeckDto,
    ) -> Result<Deck> {
        // Verify folder ownership if folder_id is provided
        if let Some(folder_id) = dto.folder_id {
            let folder_exists = sqlx::query!(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM folders
                    WHERE id = $1 AND user_id = $2
                ) as "exists!"
                "#,
                folder_id,
                user_id
            )
            .fetch_one(db)
            .await?
            .exists;

            if !folder_exists {
                return Err(AppError::BadRequest("Invalid folder ID".to_string()));
            }
        }

        let deck = sqlx::query_as!(
            Deck,
            r#"
            INSERT INTO decks (owner_id, folder_id, title, description, is_public)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, folder_id, owner_id as user_id, title as name, description, is_public, created_at, updated_at
            "#,
            user_id,
            dto.folder_id,
            dto.name,
            dto.description,
            dto.is_public.unwrap_or(false)
        )
        .fetch_one(db)
        .await?;

        Ok(deck)
    }

    pub async fn get_deck(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<Deck> {
        let deck = sqlx::query_as!(
            Deck,
            r#"
            SELECT id, folder_id, owner_id as user_id, title as name, description, is_public, created_at, updated_at
            FROM decks
            WHERE id = $1 AND (owner_id = $2 OR is_public = true)
            "#,
            id,
            user_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        Ok(deck)
    }

    pub async fn get_deck_with_stats(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<DeckWithStats> {
        let deck_stats = sqlx::query!(
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
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = $2
            WHERE d.id = $1 AND (d.owner_id = $2 OR d.is_public = true)
            GROUP BY d.id
            "#,
            id,
            user_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        Ok(DeckWithStats {
            deck: Deck {
                id: deck_stats.id,
                folder_id: deck_stats.folder_id,
                user_id: deck_stats.user_id,
                name: deck_stats.name,
                description: deck_stats.description,
                is_public: deck_stats.is_public,
                created_at: deck_stats.created_at,
                updated_at: deck_stats.updated_at,
            },
            card_count: deck_stats.card_count,
            last_studied: deck_stats.last_studied,
        })
    }

    pub async fn update_deck(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
        dto: UpdateDeckDto,
    ) -> Result<Deck> {
        // Verify ownership
        let existing = sqlx::query!(
            r#"
            SELECT owner_id as user_id
            FROM decks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        if existing.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        // Verify folder ownership if folder_id is being updated
        if let Some(folder_id) = dto.folder_id {
            let folder_exists = sqlx::query!(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM folders
                    WHERE id = $1 AND user_id = $2
                ) as "exists!"
                "#,
                folder_id,
                user_id
            )
            .fetch_one(db)
            .await?
            .exists;

            if !folder_exists {
                return Err(AppError::BadRequest("Invalid folder ID".to_string()));
            }
        }

        let deck = sqlx::query_as!(
            Deck,
            r#"
            UPDATE decks
            SET 
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                folder_id = COALESCE($5, folder_id),
                is_public = COALESCE($6, is_public)
            WHERE id = $1 AND owner_id = $2
            RETURNING id, folder_id, owner_id as user_id, title as name, description, is_public, created_at, updated_at
            "#,
            id,
            user_id,
            dto.name,
            dto.description,
            dto.folder_id,
            dto.is_public
        )
        .fetch_one(db)
        .await?;

        Ok(deck)
    }

    pub async fn delete_deck(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM decks
            WHERE id = $1 AND owner_id = $2
            "#,
            id,
            user_id
        )
        .execute(db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Resource not found".to_string()));
        }

        Ok(())
    }

    pub async fn import_csv(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
        csv_content: String,
    ) -> Result<Vec<Card>> {
        // Verify deck ownership
        let deck = Self::get_deck(db, deck_id, user_id).await?;
        if deck.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        // Parse CSV
        let mut reader = Reader::from_reader(Cursor::new(csv_content));
        let mut cards = Vec::new();
        let mut position = 0;

        // Get the current max position
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

        position = max_position + 1;

        for result in reader.deserialize::<CsvCard>() {
            let csv_card = result.map_err(|e| AppError::CsvError(e.to_string()))?;

            let card = sqlx::query_as!(
                Card,
                r#"
                INSERT INTO cards (deck_id, front, back, position)
                VALUES ($1, $2, $3, $4)
                RETURNING id, deck_id, front, back, position, created_at, updated_at
                "#,
                deck_id,
                csv_card.front,
                csv_card.back,
                position
            )
            .fetch_one(db)
            .await?;

            cards.push(card);
            position += 1;
        }

        Ok(cards)
    }

    pub async fn export_csv(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
    ) -> Result<String> {
        // Verify deck access (owner or public)
        let deck = Self::get_deck(db, deck_id, user_id).await?;

        // Get all cards for the deck
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

        // Create CSV
        let mut writer = Writer::from_writer(vec![]);
        
        // Write header
        writer.write_record(&["front", "back"])
            .map_err(|e| AppError::CsvError(e.to_string()))?;

        // Write cards
        for card in cards {
            writer.write_record(&[card.front, card.back])
                .map_err(|e| AppError::CsvError(e.to_string()))?;
        }

        let csv_data = writer.into_inner()
            .map_err(|e| AppError::CsvError(e.to_string()))?;

        String::from_utf8(csv_data)
            .map_err(|e| AppError::CsvError(e.to_string()))
    }
}
