use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    middleware::auth::UserId,
    state::AppState,
    utils::Result,
};

#[derive(Deserialize)]
struct ProgressQuery {
    deck_id: Option<Uuid>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct ProgressOverview {
    total_cards_studied: i64,
    total_study_time_minutes: i64,
    average_accuracy: f64,
    streak_days: i32,
    total_sessions: i64,
    decks_in_progress: i64,
}

#[derive(Serialize)]
struct DeckProgress {
    deck_id: Uuid,
    deck_name: String,
    total_cards: i64,
    cards_learned: i64,
    cards_reviewing: i64,
    cards_new: i64,
    average_accuracy: f64,
    last_studied: Option<DateTime<Utc>>,
    mastery_percentage: f64,
}

#[derive(Serialize)]
struct CardPerformance {
    card_id: Uuid,
    front: String,
    total_reviews: i64,
    correct_count: i64,
    incorrect_count: i64,
    accuracy_rate: f64,
    average_response_time_ms: Option<i32>,
    last_reviewed: Option<DateTime<Utc>>,
    difficulty_score: f64,
}

#[derive(Serialize)]
struct LearningCurve {
    date: DateTime<Utc>,
    cards_studied: i64,
    accuracy: f64,
    study_time_minutes: i64,
}

#[derive(Serialize)]
struct StudyStreak {
    current_streak: i32,
    longest_streak: i32,
    last_study_date: Option<DateTime<Utc>>,
    study_days: Vec<DateTime<Utc>>,
}

#[derive(Serialize)]
struct WeeklyProgress {
    week_start: DateTime<Utc>,
    total_cards_studied: i64,
    total_study_time_minutes: i64,
    average_accuracy: f64,
    sessions_completed: i64,
    new_cards_learned: i64,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/overview", get(get_progress_overview))
        .route("/decks", get(get_deck_progress))
        .route("/decks/:deck_id", get(get_specific_deck_progress))
        .route("/cards/performance", get(get_card_performance))
        .route("/learning-curve", get(get_learning_curve))
        .route("/streaks", get(get_study_streaks))
        .route("/weekly", get(get_weekly_progress))
}

async fn get_progress_overview(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<ProgressQuery>,
) -> Result<Json<ProgressOverview>> {
    let overview = sqlx::query_as!(
        ProgressOverview,
        r#"
        SELECT 
            COALESCE(COUNT(DISTINCT cp.id)::bigint, 0) as "total_cards_studied!",
            COALESCE(SUM(CASE WHEN ss.completed_at IS NOT NULL 
                THEN EXTRACT(EPOCH FROM (ss.completed_at - ss.started_at)) / 60
                ELSE 0 END)::bigint, 0) as "total_study_time_minutes!",
            COALESCE(AVG(CASE 
                WHEN cp.status = 'easy' THEN 100.0
                WHEN cp.status = 'medium' THEN 75.0
                WHEN cp.status = 'hard' THEN 50.0
                ELSE 0.0
            END)::DOUBLE PRECISION, 0.0) as "average_accuracy!",
            COALESCE(
                (SELECT current_streak FROM user_stats WHERE user_id = $1),
                0
            ) as "streak_days!",
            COUNT(DISTINCT ss.id)::bigint as "total_sessions!",
            COUNT(DISTINCT d.id)::bigint as "decks_in_progress!"
        FROM study_sessions ss
        LEFT JOIN card_progress cp ON cp.session_id = ss.id
        LEFT JOIN decks d ON d.id = ss.deck_id
        WHERE ss.user_id = $1
            AND ($2::uuid IS NULL OR ss.deck_id = $2)
            AND ($3::timestamptz IS NULL OR ss.started_at >= $3)
            AND ($4::timestamptz IS NULL OR ss.started_at <= $4)
        "#,
        user_id,
        query.deck_id,
        query.start_date,
        query.end_date
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(overview))
}

async fn get_deck_progress(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<DeckProgress>>> {
    let progress = sqlx::query_as!(
        DeckProgress,
        r#"
        WITH deck_stats AS (
            SELECT 
                d.id as deck_id,
                d.title as deck_name,
                COUNT(DISTINCT c.id) as total_cards,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IN ('easy', 'medium') AND cp.review_count >= 3
                    THEN c.id 
                END) as cards_learned,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IS NOT NULL AND cp.review_count BETWEEN 1 AND 2 
                    THEN c.id 
                END) as cards_reviewing,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IS NULL 
                    THEN c.id 
                END) as cards_new,
                AVG(CASE 
                    WHEN cp.status = 'easy' THEN 100.0
                    WHEN cp.status = 'medium' THEN 75.0
                    WHEN cp.status = 'hard' THEN 50.0
                    ELSE NULL
                END) as average_accuracy,
                MAX(ss.started_at) as last_studied
            FROM decks d
            INNER JOIN cards c ON c.deck_id = d.id
            LEFT JOIN (
                SELECT DISTINCT ON (card_id) * 
                FROM card_progress 
                ORDER BY card_id, created_at DESC
            ) cp ON cp.card_id = c.id
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = $1
            WHERE d.owner_id = $1
            GROUP BY d.id, d.title
        )
        SELECT 
            deck_id as "deck_id!",
            deck_name as "deck_name!",
            total_cards as "total_cards!",
            cards_learned as "cards_learned!",
            cards_reviewing as "cards_reviewing!",
            cards_new as "cards_new!",
            COALESCE(average_accuracy::DOUBLE PRECISION, 0.0) as "average_accuracy!",
            last_studied as "last_studied",
            CASE 
                WHEN total_cards > 0 
                THEN (cards_learned::DOUBLE PRECISION / total_cards::DOUBLE PRECISION) * 100.0
                ELSE 0.0 
            END::DOUBLE PRECISION as "mastery_percentage!"
        FROM deck_stats
        ORDER BY last_studied DESC NULLS LAST
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(progress))
}

async fn get_specific_deck_progress(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(deck_id): Path<Uuid>,
) -> Result<Json<DeckProgress>> {
    let progress = sqlx::query_as!(
        DeckProgress,
        r#"
        WITH deck_stats AS (
            SELECT 
                d.id as deck_id,
                d.title as deck_name,
                COUNT(DISTINCT c.id) as total_cards,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IN ('easy', 'medium') AND cp.review_count >= 3
                    THEN c.id 
                END) as cards_learned,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IS NOT NULL AND cp.review_count BETWEEN 1 AND 2 
                    THEN c.id 
                END) as cards_reviewing,
                COUNT(DISTINCT CASE 
                    WHEN cp.status IS NULL 
                    THEN c.id 
                END) as cards_new,
                AVG(CASE 
                    WHEN cp.status = 'easy' THEN 100.0
                    WHEN cp.status = 'medium' THEN 75.0
                    WHEN cp.status = 'hard' THEN 50.0
                    ELSE NULL
                END) as average_accuracy,
                MAX(ss.started_at) as last_studied
            FROM decks d
            INNER JOIN cards c ON c.deck_id = d.id
            LEFT JOIN (
                SELECT DISTINCT ON (card_id) * 
                FROM card_progress 
                ORDER BY card_id, created_at DESC
            ) cp ON cp.card_id = c.id
            LEFT JOIN study_sessions ss ON ss.deck_id = d.id AND ss.user_id = $1
            WHERE d.id = $2 AND d.owner_id = $1
            GROUP BY d.id, d.title
        )
        SELECT 
            deck_id as "deck_id!",
            deck_name as "deck_name!",
            total_cards as "total_cards!",
            cards_learned as "cards_learned!",
            cards_reviewing as "cards_reviewing!",
            cards_new as "cards_new!",
            COALESCE(average_accuracy::DOUBLE PRECISION, 0.0) as "average_accuracy!",
            last_studied as "last_studied",
            CASE 
                WHEN total_cards > 0 
                THEN (cards_learned::DOUBLE PRECISION / total_cards::DOUBLE PRECISION) * 100.0
                ELSE 0.0 
            END::DOUBLE PRECISION as "mastery_percentage!"
        FROM deck_stats
        "#,
        user_id,
        deck_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(progress))
}

async fn get_card_performance(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<ProgressQuery>,
) -> Result<Json<Vec<CardPerformance>>> {
    let performance = sqlx::query_as!(
        CardPerformance,
        r#"
        WITH card_stats AS (
            SELECT 
                c.id as card_id,
                c.front,
                COUNT(cp.id) as total_reviews,
                COUNT(CASE WHEN cp.status IN ('easy', 'medium') THEN 1 END) as correct_count,
                COUNT(CASE WHEN cp.status IN ('hard', 'forgot') THEN 1 END) as incorrect_count,
                AVG(cp.response_time_ms::float) as avg_response_time,
                MAX(cp.created_at) as last_reviewed
            FROM cards c
            INNER JOIN decks d ON d.id = c.deck_id
            LEFT JOIN card_progress cp ON cp.card_id = c.id
            WHERE d.owner_id = $1
                AND ($2::uuid IS NULL OR c.deck_id = $2)
                AND ($3::timestamptz IS NULL OR cp.created_at >= $3)
                AND ($4::timestamptz IS NULL OR cp.created_at <= $4)
            GROUP BY c.id, c.front
        )
        SELECT 
            card_id as "card_id!",
            front as "front!",
            total_reviews as "total_reviews!",
            correct_count as "correct_count!",
            incorrect_count as "incorrect_count!",
            CASE 
                WHEN total_reviews > 0 
                THEN (correct_count::DOUBLE PRECISION / total_reviews::DOUBLE PRECISION) * 100.0
                ELSE 0.0 
            END::DOUBLE PRECISION as "accuracy_rate!",
            avg_response_time::int as "average_response_time_ms",
            last_reviewed as "last_reviewed",
            CASE 
                WHEN total_reviews > 0 
                THEN 1.0 - (correct_count::DOUBLE PRECISION / total_reviews::DOUBLE PRECISION)
                ELSE 0.5
            END::DOUBLE PRECISION as "difficulty_score!"
        FROM card_stats
        WHERE total_reviews > 0
        ORDER BY 
            CASE 
                WHEN total_reviews > 0 
                THEN 1.0 - (correct_count::DOUBLE PRECISION / total_reviews::DOUBLE PRECISION)
                ELSE 0.5
            END DESC, total_reviews DESC
        LIMIT 100
        "#,
        user_id,
        query.deck_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(performance))
}

async fn get_learning_curve(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<ProgressQuery>,
) -> Result<Json<Vec<LearningCurve>>> {
    let curve = sqlx::query_as!(
        LearningCurve,
        r#"
        WITH daily_stats AS (
            SELECT 
                DATE(ss.started_at) as study_date,
                COUNT(DISTINCT cp.card_id) as cards_studied,
                AVG(CASE 
                    WHEN cp.status = 'easy' THEN 100.0
                    WHEN cp.status = 'medium' THEN 75.0
                    WHEN cp.status = 'hard' THEN 50.0
                    ELSE 0.0
                END) as accuracy,
                SUM(EXTRACT(EPOCH FROM (
                    COALESCE(ss.completed_at, ss.started_at + INTERVAL '30 minutes') 
                    - ss.started_at
                )) / 60)::bigint as study_time_minutes
            FROM study_sessions ss
            LEFT JOIN card_progress cp ON cp.session_id = ss.id
            WHERE ss.user_id = $1
                AND ($2::uuid IS NULL OR ss.deck_id = $2)
                AND ($3::timestamptz IS NULL OR ss.started_at >= $3)
                AND ($4::timestamptz IS NULL OR ss.started_at <= $4)
            GROUP BY DATE(ss.started_at)
        )
        SELECT 
            study_date::timestamptz as "date!",
            cards_studied as "cards_studied!",
            COALESCE(accuracy::DOUBLE PRECISION, 0.0) as "accuracy!",
            COALESCE(study_time_minutes, 0) as "study_time_minutes!"
        FROM daily_stats
        ORDER BY study_date DESC
        LIMIT 30
        "#,
        user_id,
        query.deck_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(curve))
}

async fn get_study_streaks(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<StudyStreak>> {
    let user_stats = sqlx::query!(
        r#"
        SELECT 
            current_streak,
            longest_streak,
            last_study_date
        FROM user_stats
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(&state.db)
    .await?;

    let study_days = sqlx::query!(
        r#"
        SELECT DISTINCT DATE(started_at)::timestamptz as study_date
        FROM study_sessions
        WHERE user_id = $1
            AND started_at >= CURRENT_DATE - INTERVAL '30 days'
        ORDER BY study_date DESC
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await?;

    let streak = StudyStreak {
        current_streak: user_stats.as_ref().map(|s| s.current_streak).unwrap_or(0),
        longest_streak: user_stats.as_ref().map(|s| s.longest_streak).unwrap_or(0),
        last_study_date: user_stats.and_then(|s| s.last_study_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap())),
        study_days: study_days
            .into_iter()
            .filter_map(|r| r.study_date)
            .collect(),
    };

    Ok(Json(streak))
}

async fn get_weekly_progress(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<WeeklyProgress>>> {
    let progress = sqlx::query_as!(
        WeeklyProgress,
        r#"
        WITH weekly_stats AS (
            SELECT 
                DATE_TRUNC('week', ss.started_at) as week_start,
                COUNT(DISTINCT cp.card_id) as total_cards_studied,
                SUM(EXTRACT(EPOCH FROM (
                    COALESCE(ss.completed_at, ss.started_at + INTERVAL '30 minutes') 
                    - ss.started_at
                )) / 60)::bigint as total_study_time_minutes,
                AVG(CASE 
                    WHEN cp.status = 'easy' THEN 100.0
                    WHEN cp.status = 'medium' THEN 75.0
                    WHEN cp.status = 'hard' THEN 50.0
                    ELSE 0.0
                END) as average_accuracy,
                COUNT(DISTINCT ss.id) as sessions_completed,
                COUNT(DISTINCT CASE 
                    WHEN cp.review_count = 1 
                    THEN cp.card_id 
                END) as new_cards_learned
            FROM study_sessions ss
            LEFT JOIN card_progress cp ON cp.session_id = ss.id
            WHERE ss.user_id = $1
                AND ss.started_at >= CURRENT_DATE - INTERVAL '12 weeks'
            GROUP BY DATE_TRUNC('week', ss.started_at)
        )
        SELECT 
            week_start as "week_start!",
            COALESCE(total_cards_studied, 0) as "total_cards_studied!",
            COALESCE(total_study_time_minutes, 0) as "total_study_time_minutes!",
            COALESCE(average_accuracy::DOUBLE PRECISION, 0.0) as "average_accuracy!",
            sessions_completed as "sessions_completed!",
            COALESCE(new_cards_learned, 0) as "new_cards_learned!"
        FROM weekly_stats
        ORDER BY week_start DESC
        LIMIT 12
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(progress))
}
