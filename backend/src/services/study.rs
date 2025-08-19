use crate::{
    models::{
        Achievement, AchievementWithStatus, CardProgress, CardStatus, CreateStudySessionDto,
        StudySession, SubmitCardAnswerDto, UpdateStudySessionDto, UserAchievement, UserCardStats,
        UserStats,
    },
    utils::{AppError, Result},
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct StudyService;

impl StudyService {
    pub async fn create_study_session(
        db: &PgPool,
        user_id: Uuid,
        dto: CreateStudySessionDto,
    ) -> Result<StudySession> {
        // Verify deck access
        let deck_access = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM decks
                WHERE id = $1 AND owner_id = $2
            ) as "exists!"
            "#,
            dto.deck_id,
            user_id
        )
        .fetch_one(db)
        .await?
        .exists;

        if !deck_access {
            return Err(AppError::NotFound("Resource not found".to_string()));
        }

        let session = sqlx::query_as!(
            StudySession,
            r#"
            INSERT INTO study_sessions (user_id, deck_id, study_mode)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, deck_id, study_mode, total_cards, cards_studied, 
                     cards_correct, cards_incorrect, cards_skipped, duration_seconds,
                     started_at, completed_at, created_at, updated_at
            "#,
            user_id,
            dto.deck_id,
            dto.study_mode.as_deref().unwrap_or("standard")
        )
        .fetch_one(db)
        .await?;

        Ok(session)
    }

    pub async fn get_study_session(
        db: &PgPool,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<StudySession> {
        let session = sqlx::query_as!(
            StudySession,
            r#"
            SELECT id, user_id, deck_id, study_mode, total_cards, cards_studied,
                   cards_correct, cards_incorrect, cards_skipped, duration_seconds,
                   started_at, completed_at, created_at, updated_at
            FROM study_sessions
            WHERE id = $1 AND user_id = $2
            "#,
            session_id,
            user_id
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::NotFound("Resource not found".to_string()))?;

        Ok(session)
    }

    pub async fn record_card_progress(
        db: &PgPool,
        session_id: Uuid,
        card_id: Uuid,
        user_id: Uuid,
        status: CardStatus,
        response_time_ms: Option<i32>,
    ) -> Result<CardProgress> {
        // Verify session ownership
        let session = Self::get_study_session(db, session_id, user_id).await?;

        // Verify card belongs to the deck being studied
        let card_in_deck = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM cards
                WHERE id = $1 AND deck_id = $2
            ) as "exists!"
            "#,
            card_id,
            session.deck_id
        )
        .fetch_one(db)
        .await?
        .exists;

        if !card_in_deck {
            return Err(AppError::BadRequest("Card not in study deck".to_string()));
        }

        // Record the progress
        let progress = sqlx::query_as!(
            CardProgress,
            r#"
            INSERT INTO card_progress (session_id, card_id, user_id, status, response_time_ms)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, session_id, card_id, user_id, status as "status: CardStatus", 
                     response_time_ms, user_answer, is_correct, studied_at, created_at
            "#,
            session_id,
            card_id,
            user_id,
            status as CardStatus,
            response_time_ms
        )
        .fetch_one(db)
        .await?;

        // Update session statistics
        let is_correct = matches!(status, CardStatus::Easy | CardStatus::Medium);
        
        sqlx::query!(
            r#"
            UPDATE study_sessions
            SET 
                cards_studied = cards_studied + 1,
                cards_correct = cards_correct + $2
            WHERE id = $1
            "#,
            session_id,
            if is_correct { 1 } else { 0 }
        )
        .execute(db)
        .await?;

        Ok(progress)
    }

    pub async fn complete_study_session(
        db: &PgPool,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<StudySession> {
        // Verify ownership
        let _session = Self::get_study_session(db, session_id, user_id).await?;

        let session = sqlx::query_as!(
            StudySession,
            r#"
            UPDATE study_sessions
            SET completed_at = $2, updated_at = $2
            WHERE id = $1 AND user_id = $3
            RETURNING id, user_id, deck_id, study_mode, total_cards, cards_studied,
                     cards_correct, cards_incorrect, cards_skipped, duration_seconds,
                     started_at, completed_at, created_at, updated_at
            "#,
            session_id,
            Utc::now(),
            user_id
        )
        .fetch_one(db)
        .await?;

        Ok(session)
    }

    pub async fn get_user_study_sessions(
        db: &PgPool,
        user_id: Uuid,
        limit: Option<i64>,
    ) -> Result<Vec<StudySession>> {
        let limit = limit.unwrap_or(50);
        
        let sessions = sqlx::query_as!(
            StudySession,
            r#"
            SELECT id, user_id, deck_id, study_mode, total_cards, cards_studied,
                   cards_correct, cards_incorrect, cards_skipped, duration_seconds,
                   started_at, completed_at, created_at, updated_at
            FROM study_sessions
            WHERE user_id = $1
            ORDER BY started_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(db)
        .await?;

        Ok(sessions)
    }

    pub async fn get_session_progress(
        db: &PgPool,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<CardProgress>> {
        // Verify session ownership
        let _session = Self::get_study_session(db, session_id, user_id).await?;

        let progress = sqlx::query_as!(
            CardProgress,
            r#"
            SELECT id, session_id, card_id, user_id, status as "status: CardStatus", 
                   response_time_ms, user_answer, is_correct, studied_at, created_at
            FROM card_progress
            WHERE session_id = $1
            ORDER BY studied_at
            "#,
            session_id
        )
        .fetch_all(db)
        .await?;

        Ok(progress)
    }
}
