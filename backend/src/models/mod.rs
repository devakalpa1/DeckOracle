pub mod ai;
pub mod import_export;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// User model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub display_name: Option<String>,
}

// Authentication DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenDto {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PasswordResetRequestDto {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PasswordResetDto {
    pub token: String,
    #[validate(length(min = 8, max = 128))]
    #[validate(custom(function = "validate_password_strength"))]
    pub new_password: String,
}

// Custom password validation
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !(has_uppercase && has_lowercase && has_digit) {
        return Err(validator::ValidationError::new("weak_password"));
    }
    
    Ok(())
}

// Folder model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub parent_folder_id: Option<Uuid>,
    pub name: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFolderDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub parent_folder_id: Option<Uuid>,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFolderDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub parent_folder_id: Option<Uuid>,
    pub position: Option<i32>,
}

// Deck model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Deck {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    #[sqlx(rename = "owner_id")]
    pub user_id: Uuid,  // Keep as user_id in the API but map to owner_id in DB
    #[sqlx(rename = "title")]
    pub name: String,   // Keep as name in the API but map to title in DB
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDeckDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub folder_id: Option<Uuid>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDeckDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub folder_id: Option<Uuid>,
    pub is_public: Option<bool>,
}

// Card model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: Uuid,
    pub deck_id: Uuid,
    pub front: String,
    pub back: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCardDto {
    #[validate(length(min = 1))]
    pub front: String,
    #[validate(length(min = 1))]
    pub back: String,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateCardDto {
    pub front: Option<String>,
    pub back: Option<String>,
    pub position: Option<i32>,
}

// CSV import/export DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvCard {
    pub front: String,
    pub back: String,
}

// Study session models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StudySession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub deck_id: Uuid,
    pub study_mode: String,
    pub total_cards: i32,
    pub cards_studied: i32,
    pub cards_correct: i32,
    pub cards_incorrect: i32,
    pub cards_skipped: i32,
    pub duration_seconds: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateStudySessionDto {
    pub deck_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub study_mode: Option<String>, // standard, quiz, timed, custom
    pub card_ids: Option<Vec<Uuid>>, // For custom study sessions
    pub time_limit_seconds: Option<i32>, // For timed sessions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStudySessionDto {
    pub cards_studied: Option<i32>,
    pub cards_correct: Option<i32>,
    pub cards_incorrect: Option<i32>,
    pub cards_skipped: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
}

// Card progress model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardProgress {
    pub id: Uuid,
    pub session_id: Uuid,
    pub card_id: Uuid,
    pub user_id: Uuid,
    pub status: CardStatus,
    pub response_time_ms: Option<i32>,
    pub user_answer: Option<String>,
    pub is_correct: Option<bool>,
    pub studied_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCardAnswerDto {
    pub card_id: Uuid,
    pub status: CardStatus,
    pub response_time_ms: Option<i32>,
    pub user_answer: Option<String>,
    pub is_correct: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "card_status", rename_all = "lowercase")]
pub enum CardStatus {
    Easy,
    Medium,
    Hard,
    Forgot,
}

// User statistics and gamification
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserStats {
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_cards_studied: i32,
    pub total_study_time_seconds: i32,
    pub current_streak_days: i32,
    pub longest_streak_days: i32,
    pub last_study_date: Option<chrono::NaiveDate>,
    pub total_points: i32,
    pub level: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserCardStats {
    pub id: Uuid,
    pub user_id: Uuid,
    pub card_id: Uuid,
    pub times_seen: i32,
    pub times_correct: i32,
    pub times_incorrect: i32,
    pub average_response_time_ms: Option<i32>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub difficulty_rating: Option<f32>,
    pub next_review_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Achievement models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Achievement {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon_name: Option<String>,
    pub points: i32,
    pub criteria_type: String,
    pub criteria_value: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAchievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement_id: Uuid,
    pub earned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementWithStatus {
    #[serde(flatten)]
    pub achievement: Achievement,
    pub earned: bool,
    pub earned_at: Option<DateTime<Utc>>,
    pub progress: Option<i32>, // Current progress towards achievement
}

// Response DTOs with counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckWithStats {
    #[serde(flatten)]
    pub deck: Deck,
    pub card_count: i64,
    pub last_studied: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderWithContents {
    #[serde(flatten)]
    pub folder: Folder,
    pub subfolders: Vec<Folder>,
    pub decks: Vec<DeckWithStats>,
}
