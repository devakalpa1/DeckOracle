use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::UserId,
    models::{CreateFolderDto, Folder, FolderWithContents, UpdateFolderDto},
    services::folder::FolderService,
    state::AppState,
    utils::{AppError, Result},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_folders).post(create_folder))
        .route("/:id", get(get_folder).patch(update_folder).delete(delete_folder))
        .route("/:id/contents", get(get_folder_contents))
}

async fn list_folders(
    State(state): State<AppState>,
    UserId(user_id): UserId,
) -> Result<Json<Vec<Folder>>> {
    let folders = FolderService::list_user_folders(&state.db, user_id).await?;
    Ok(Json(folders))
}

async fn create_folder(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Json(dto): Json<CreateFolderDto>,
) -> Result<(StatusCode, Json<Folder>)> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let folder = FolderService::create_folder(&state.db, user_id, dto).await?;
    Ok((StatusCode::CREATED, Json(folder)))
}

async fn get_folder(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Folder>> {
    // TODO: Get user_id from auth middleware and verify ownership
    let user_id = Uuid::new_v4(); // Placeholder
    
    let folder = FolderService::get_folder(&state.db, id, user_id).await?;
    Ok(Json(folder))
}

async fn update_folder(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateFolderDto>,
) -> Result<Json<Folder>> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // TODO: Get user_id from auth middleware and verify ownership
    let user_id = Uuid::new_v4(); // Placeholder
    
    let folder = FolderService::update_folder(&state.db, id, user_id, dto).await?;
    Ok(Json(folder))
}

async fn delete_folder(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    // TODO: Get user_id from auth middleware and verify ownership
    let user_id = Uuid::new_v4(); // Placeholder
    
    FolderService::delete_folder(&state.db, id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_folder_contents(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<FolderWithContents>> {
    // TODO: Get user_id from auth middleware and verify ownership
    let user_id = Uuid::new_v4(); // Placeholder
    
    let contents = FolderService::get_folder_with_contents(&state.db, id, user_id).await?;
    Ok(Json(contents))
}
