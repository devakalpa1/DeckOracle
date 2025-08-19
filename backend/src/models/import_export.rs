use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Json,
    Csv,
    Anki,
    Markdown,
}

// Import formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    Json,
    Csv,
    Anki,
    Markdown,
}

// Export request DTOs
#[derive(Debug, Deserialize)]
pub struct ExportDeckRequest {
    pub deck_id: Uuid,
    pub format: ExportFormat,
    pub include_progress: Option<bool>,
    pub include_media: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BulkExportRequest {
    pub deck_ids: Vec<Uuid>,
    pub format: ExportFormat,
    pub include_progress: Option<bool>,
    pub include_media: Option<bool>,
}

// Import request DTOs
#[derive(Debug, Deserialize)]
pub struct ImportDeckRequest {
    pub format: ImportFormat,
    pub folder_id: Option<Uuid>,
    pub merge_duplicates: Option<bool>,
}

// Export data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedDeck {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cards: Vec<ExportedCard>,
    pub metadata: ExportMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedCard {
    pub id: Uuid,
    pub front: String,
    pub back: String,
    pub explanation: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: Option<i32>,
    pub media: Vec<MediaAttachment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub progress: Option<CardProgressData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaAttachment {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub data: Option<String>, // Base64 encoded
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardProgressData {
    pub review_count: i32,
    pub correct_count: i32,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub next_review: Option<DateTime<Utc>>,
    pub ease_factor: f32,
    pub interval_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub version: String,
    pub exported_at: DateTime<Utc>,
    pub platform: String,
    pub format: String,
    pub total_cards: usize,
    pub includes_progress: bool,
    pub includes_media: bool,
}

// CSV export structures
#[derive(Debug, Serialize, Deserialize)]
pub struct CsvCard {
    pub front: String,
    pub back: String,
    pub tags: String, // Comma-separated
    pub explanation: String,
    pub difficulty: Option<i32>,
}

// Anki export structures
#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiDeck {
    pub name: String,
    pub desc: String,
    pub cards: Vec<AnkiCard>,
    pub notes: Vec<AnkiNote>,
    pub models: Vec<AnkiModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiCard {
    pub nid: i64, // Note ID
    pub ord: i32, // Card ordinal
    pub did: i64, // Deck ID
    pub due: i64,
    pub ivl: i32, // Interval
    pub factor: i32, // Ease factor * 1000
    pub reps: i32, // Review count
    pub lapses: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiNote {
    pub id: i64,
    pub guid: String,
    pub mid: i64, // Model ID
    pub fields: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiModel {
    pub id: i64,
    pub name: String,
    pub flds: Vec<AnkiField>,
    pub tmpls: Vec<AnkiTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiField {
    pub name: String,
    pub ord: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiTemplate {
    pub name: String,
    pub qfmt: String, // Question format
    pub afmt: String, // Answer format
}

// Import validation
#[derive(Debug, Serialize)]
pub struct ImportValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub deck_count: usize,
    pub card_count: usize,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub success: bool,
    pub imported_decks: Vec<ImportedDeck>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub total_cards_imported: usize,
    pub total_decks_imported: usize,
}

#[derive(Debug, Serialize)]
pub struct ImportedDeck {
    pub id: Uuid,
    pub title: String,
    pub card_count: usize,
    pub was_merged: bool,
}
