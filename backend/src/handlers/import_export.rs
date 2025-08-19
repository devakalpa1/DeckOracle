use axum::{
    body::Bytes,
    extract::{Multipart, Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::auth::UserId,
    models::import_export::*,
    services::import_export::ImportExportService,
    state::AppState,
    utils::Result,
};

#[derive(Deserialize)]
struct ExportQuery {
    format: ExportFormat,
    include_progress: Option<bool>,
    include_media: Option<bool>,
}

#[derive(Deserialize)]
struct BulkExportQuery {
    deck_ids: String, // Comma-separated UUIDs
    format: ExportFormat,
    include_progress: Option<bool>,
    include_media: Option<bool>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/export/:deck_id", get(export_deck))
        .route("/export/bulk", get(export_bulk))
        .route("/import", post(import_deck))
        .route("/import/validate", post(validate_import))
        .route("/templates/:format", get(get_import_template))
}

// Export a single deck
async fn export_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Path(deck_id): Path<Uuid>,
    Query(query): Query<ExportQuery>,
) -> Result<Response> {
    let data = ImportExportService::export_deck(
        &state.db,
        user_id,
        deck_id,
        query.format.clone(),
        query.include_progress.unwrap_or(false),
        query.include_media.unwrap_or(false),
    )
    .await?;

    let (content_type, file_extension) = match query.format {
        ExportFormat::Json => ("application/json", "json"),
        ExportFormat::Csv => ("text/csv", "csv"),
        ExportFormat::Anki => ("application/json", "json"), // Would be .apkg in production
        ExportFormat::Markdown => ("text/markdown", "md"),
    };

    let filename = format!("deck_{}.{}", deck_id, file_extension);
    
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", filename).parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, data).into_response())
}

// Export multiple decks
async fn export_bulk(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    Query(query): Query<BulkExportQuery>,
) -> Result<Response> {
    // Parse deck IDs from comma-separated string
    let deck_ids: Vec<Uuid> = query
        .deck_ids
        .split(',')
        .filter_map(|id| id.trim().parse().ok())
        .collect();

    if deck_ids.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No valid deck IDs provided"
            }))
        ).into_response());
    }

    let data = ImportExportService::export_decks(
        &state.db,
        user_id,
        deck_ids,
        query.format.clone(),
        query.include_progress.unwrap_or(false),
        query.include_media.unwrap_or(false),
    )
    .await?;

    let (content_type, file_extension) = match query.format {
        ExportFormat::Json => ("application/json", "json"),
        ExportFormat::Csv => ("text/csv", "csv"),
        ExportFormat::Anki => ("application/json", "json"),
        ExportFormat::Markdown => ("text/markdown", "md"),
    };

    let filename = format!("decks_export.{}", file_extension);
    
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", filename).parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, data).into_response())
}

// Import deck from uploaded file
async fn import_deck(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    mut multipart: Multipart,
) -> Result<Json<ImportResult>> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut format: Option<ImportFormat> = None;
    let mut folder_id: Option<Uuid> = None;
    let mut merge_duplicates = false;

    // Process multipart form data
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "file" => {
                let data = field.bytes().await?;
                file_data = Some(data.to_vec());
            }
            "format" => {
                let value = field.text().await?;
                format = match value.as_str() {
                    "json" => Some(ImportFormat::Json),
                    "csv" => Some(ImportFormat::Csv),
                    "anki" => Some(ImportFormat::Anki),
                    "markdown" => Some(ImportFormat::Markdown),
                    _ => None,
                };
            }
            "folder_id" => {
                let value = field.text().await?;
                folder_id = value.parse().ok();
            }
            "merge_duplicates" => {
                let value = field.text().await?;
                merge_duplicates = value.parse().unwrap_or(false);
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| {
        crate::utils::error::AppError::BadRequest("No file provided".to_string())
    })?;
    
    let format = format.ok_or_else(|| {
        crate::utils::error::AppError::BadRequest("No format specified".to_string())
    })?;

    let result = ImportExportService::import_decks(
        &state.db,
        user_id,
        file_data,
        format,
        folder_id,
        merge_duplicates,
    )
    .await?;

    Ok(Json(result))
}

// Validate import file without actually importing
async fn validate_import(
    State(state): State<AppState>,
    UserId(user_id): UserId,
    mut multipart: Multipart,
) -> Result<Json<ImportValidationResult>> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut format: Option<ImportFormat> = None;

    // Process multipart form data
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "file" => {
                let data = field.bytes().await?;
                file_data = Some(data.to_vec());
            }
            "format" => {
                let value = field.text().await?;
                format = match value.as_str() {
                    "json" => Some(ImportFormat::Json),
                    "csv" => Some(ImportFormat::Csv),
                    "anki" => Some(ImportFormat::Anki),
                    "markdown" => Some(ImportFormat::Markdown),
                    _ => None,
                };
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| {
        crate::utils::error::AppError::BadRequest("No file provided".to_string())
    })?;
    
    let format = format.ok_or_else(|| {
        crate::utils::error::AppError::BadRequest("No format specified".to_string())
    })?;

    // Use the validate_import function from the service
    let validation = ImportExportService::validate_import(&file_data, &format)?;
    
    Ok(Json(validation))
}

// Get import template for a specific format
async fn get_import_template(
    Path(format): Path<String>,
) -> Result<Response> {
    let (template_data, content_type, file_extension) = match format.as_str() {
        "json" => {
            let template = serde_json::json!({
                "title": "Sample Deck",
                "description": "Description of your deck",
                "cards": [
                    {
                        "front": "Question 1",
                        "back": "Answer 1",
                        "tags": ["tag1", "tag2"]
                    },
                    {
                        "front": "Question 2",
                        "back": "Answer 2",
                        "tags": ["tag3"]
                    }
                ]
            });
            (
                serde_json::to_vec_pretty(&template)?,
                "application/json",
                "json"
            )
        }
        "csv" => {
            let template = b"Front,Back,Tags,Explanation,Difficulty\n\
                Question 1,Answer 1,\"tag1,tag2\",Optional explanation,1\n\
                Question 2,Answer 2,tag3,Another explanation,2\n";
            (template.to_vec(), "text/csv", "csv")
        }
        "markdown" => {
            let template = b"# Deck Title\n\n\
                Optional deck description goes here.\n\n\
                ---\n\n\
                ## Card 1\n\n\
                **Front:** Question 1\n\n\
                **Back:** Answer 1\n\n\
                ---\n\n\
                ## Card 2\n\n\
                **Front:** Question 2\n\n\
                **Back:** Answer 2\n\n\
                ---\n";
            (template.to_vec(), "text/markdown", "md")
        }
        _ => {
            return Ok((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid format specified"
                }))
            ).into_response());
        }
    };

    let filename = format!("import_template.{}", file_extension);
    
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", filename).parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, template_data).into_response())
}

