use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::auth::UserId,
    models::{CardProgress, CardStatus, CreateStudySessionDto, StudySession},
    services::study::StudyService,
    state::AppState,
    utils::Result,
};

#[derive(Deserialize)]
struct StudySessionsQuery {
    limit: Option<i64>,
}

#[derive(Deserialize)]
struct RecordProgressDto {
    card_id: Uuid,
    status: CardStatus,
    response_time_ms: Option<i32>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sessions", get(list_sessions).post(create_session))
        .route("/sessions/:id", get(get_session))
        .route("/sessions/:id/complete", post(complete_session))
        .route("/sessions/:id/progress", get(get_session_progress).post(record_progress))
}

async fn list_sessions(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<StudySessionsQuery>,
) -> Result<Json<Vec<StudySession>>> {
    let sessions = StudyService::get_user_study_sessions(&state.db, user_id, query.limit).await?;
    Ok(Json(sessions))
}

async fn create_session(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Json(dto): Json<CreateStudySessionDto>,
) -> Result<(StatusCode, Json<StudySession>)> {
    let session = StudyService::create_study_session(&state.db, user_id, dto).await?;
    Ok((StatusCode::CREATED, Json(session)))
}

async fn get_session(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<StudySession>> {
    let session = StudyService::get_study_session(&state.db, id, user_id).await?;
    Ok(Json(session))
}

async fn complete_session(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<StudySession>> {
    let session = StudyService::complete_study_session(&state.db, id, user_id).await?;
    Ok(Json(session))
}

async fn get_session_progress(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<CardProgress>>> {
    let progress = StudyService::get_session_progress(&state.db, id, user_id).await?;
    Ok(Json(progress))
}

async fn record_progress(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(session_id): Path<Uuid>,
    Json(dto): Json<RecordProgressDto>,
) -> Result<(StatusCode, Json<CardProgress>)> {
    let progress = StudyService::record_card_progress(
        &state.db,
        session_id,
        dto.card_id,
        user_id,
        dto.status,
        dto.response_time_ms,
    )
    .await?;
    
    Ok((StatusCode::CREATED, Json(progress)))
}
