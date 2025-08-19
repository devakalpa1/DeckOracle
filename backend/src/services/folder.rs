use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{CreateFolderDto, Deck, DeckWithStats, Folder, FolderWithContents, UpdateFolderDto},
    utils::{AppError, Result},
};

pub struct FolderService;

impl FolderService {
    pub async fn list_user_folders(db: &PgPool, user_id: Uuid) -> Result<Vec<Folder>> {
        let folders = sqlx::query_as!(
            Folder,
            r#"
            SELECT id, user_id, parent_folder_id, name, position, created_at, updated_at
            FROM folders
            WHERE user_id = $1
            ORDER BY parent_folder_id NULLS FIRST, position, name
            "#,
            user_id
        )
        .fetch_all(db)
        .await?;

        Ok(folders)
    }

    pub async fn create_folder(
        db: &PgPool,
        user_id: Uuid,
        dto: CreateFolderDto,
    ) -> Result<Folder> {
        // Get the next position if not provided
        let position = match dto.position {
            Some(pos) => pos,
            None => {
                let max_position = sqlx::query!(
                    r#"
                    SELECT COALESCE(MAX(position), -1) as "max_position!"
                    FROM folders
                    WHERE user_id = $1 
                    AND (parent_folder_id = $2 OR (parent_folder_id IS NULL AND $2 IS NULL))
                    "#,
                    user_id,
                    dto.parent_folder_id
                )
                .fetch_one(db)
                .await?
                .max_position;

                max_position + 1
            }
        };

        let folder = sqlx::query_as!(
            Folder,
            r#"
            INSERT INTO folders (user_id, parent_folder_id, name, position)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, parent_folder_id, name, position, created_at, updated_at
            "#,
            user_id,
            dto.parent_folder_id,
            dto.name,
            position
        )
        .fetch_one(db)
        .await?;

        Ok(folder)
    }

    pub async fn get_folder(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<Folder> {
        let folder = sqlx::query_as!(
            Folder,
            r#"
            SELECT id, user_id, parent_folder_id, name, position, created_at, updated_at
            FROM folders
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        Ok(folder)
    }

    pub async fn update_folder(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
        dto: UpdateFolderDto,
    ) -> Result<Folder> {
        // First check if folder exists and belongs to user
        let _existing = Self::get_folder(db, id, user_id).await?;

        let folder = sqlx::query_as!(
            Folder,
            r#"
            UPDATE folders
            SET 
                name = COALESCE($3, name),
                parent_folder_id = COALESCE($4, parent_folder_id),
                position = COALESCE($5, position)
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, parent_folder_id, name, position, created_at, updated_at
            "#,
            id,
            user_id,
            dto.name,
            dto.parent_folder_id,
            dto.position
        )
        .fetch_one(db)
        .await?;

        Ok(folder)
    }

    pub async fn delete_folder(db: &PgPool, id: Uuid, user_id: Uuid) -> Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM folders
            WHERE id = $1 AND user_id = $2
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

    pub async fn get_folder_with_contents(
        db: &PgPool,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<FolderWithContents> {
        // Get the folder
        let folder = Self::get_folder(db, id, user_id).await?;

        // Get subfolders
        let subfolders = sqlx::query_as!(
            Folder,
            r#"
            SELECT id, user_id, parent_folder_id, name, position, created_at, updated_at
            FROM folders
            WHERE parent_folder_id = $1 AND user_id = $2
            ORDER BY position, name
            "#,
            id,
            user_id
        )
        .fetch_all(db)
        .await?;

        // Get decks with stats
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
            WHERE d.folder_id = $1 AND d.owner_id = $2
            GROUP BY d.id
            ORDER BY d.title
            "#,
            id,
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

        Ok(FolderWithContents {
            folder,
            subfolders,
            decks,
        })
    }
}
