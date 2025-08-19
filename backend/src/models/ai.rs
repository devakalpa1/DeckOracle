use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ============== Study Events & Analytics ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StudyEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub card_id: Uuid,
    pub deck_id: Uuid,
    pub session_id: Option<Uuid>,
    pub event_type: String, // 'view', 'answer', 'skip', 'review'
    pub outcome: Option<String>, // 'correct', 'incorrect', 'partial', 'skipped'
    pub response_time_ms: Option<i32>,
    pub confidence_rating: Option<i32>,
    pub ease_factor: f32,
    pub interval_days: i32,
    pub repetition_number: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateStudyEventDto {
    pub card_id: Uuid,
    pub deck_id: Uuid,
    pub session_id: Option<Uuid>,
    #[validate(length(min = 1, max = 50))]
    pub event_type: String,
    pub outcome: Option<String>,
    pub response_time_ms: Option<i32>,
    #[validate(range(min = 1, max = 5))]
    pub confidence_rating: Option<i32>,
}

// ============== AI Privacy Settings ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiPrivacySettings {
    pub user_id: Uuid,
    pub track_analytics: bool,
    pub enable_ai_recommendations: bool,
    pub enable_content_generation: bool,
    pub share_anonymous_data: bool,
    pub personalized_learning: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePrivacySettingsDto {
    pub track_analytics: Option<bool>,
    pub enable_ai_recommendations: Option<bool>,
    pub enable_content_generation: Option<bool>,
    pub share_anonymous_data: Option<bool>,
    pub personalized_learning: Option<bool>,
}

// ============== AI Recommendations ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiRecommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recommendation_type: String, // 'next_card', 'study_time', 'deck_suggestion', 'review_schedule'
    pub payload: JsonValue,
    pub confidence_score: Option<f32>,
    pub shown_at: Option<DateTime<Utc>>,
    pub accepted: Option<bool>,
    pub feedback: Option<String>, // 'helpful', 'not_helpful', 'ignored'
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationPayload {
    pub title: String,
    pub description: String,
    pub action_type: String,
    pub action_data: JsonValue,
    pub reason: Option<String>,
    pub metrics: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RecommendationFeedbackDto {
    pub recommendation_id: Uuid,
    #[validate(length(min = 1, max = 20))]
    pub feedback: String, // 'helpful', 'not_helpful', 'ignored'
    pub accepted: bool,
}

// ============== Content Generation ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiContentGenerationJob {
    pub id: Uuid,
    pub user_id: Uuid,
    pub deck_id: Option<Uuid>,
    pub job_type: String, // 'pdf_extract', 'docx_extract', 'summarize', 'generate_questions'
    pub status: String, // 'pending', 'processing', 'completed', 'failed'
    pub input_file_path: Option<String>,
    pub input_metadata: Option<JsonValue>,
    pub output_data: Option<JsonValue>,
    pub error_message: Option<String>,
    pub provider: Option<String>, // 'vertex_ai', 'local_model', 'openai'
    pub model_name: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateContentGenerationJobDto {
    pub deck_id: Option<Uuid>,
    #[validate(length(min = 1, max = 50))]
    pub job_type: String,
    pub input_metadata: Option<JsonValue>,
    #[validate(length(min = 1, max = 50))]
    pub provider: Option<String>,
    pub model_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGenerationRequest {
    pub deck_id: Option<Uuid>,
    pub content_type: String, // 'pdf', 'docx', 'txt', 'csv', 'doc'
    pub generation_mode: String, // 'extract', 'summarize', 'qa_pairs', 'smart'
    pub options: ContentGenerationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGenerationOptions {
    pub max_cards: Option<i32>,
    pub difficulty_level: Option<String>, // 'easy', 'medium', 'hard', 'mixed'
    pub include_explanations: Option<bool>,
    pub language: Option<String>,
    pub card_format: Option<String>, // 'question_answer', 'term_definition', 'concept_explanation'
    pub custom_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AiGeneratedCard {
    pub id: Uuid,
    pub job_id: Uuid,
    pub deck_id: Option<Uuid>,
    pub front: String,
    pub back: String,
    pub explanation: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty_estimate: Option<i32>,
    pub confidence_score: Option<f32>,
    pub source_context: Option<String>,
    pub approved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ApproveGeneratedCardsDto {
    pub card_ids: Vec<Uuid>,
    pub deck_id: Uuid,
    pub auto_position: Option<bool>, // Auto-assign positions
}

// ============== User Card Statistics ==============

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
    pub ease_factor: f32,
    pub interval_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============== Learning Patterns ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LearningPattern {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pattern_type: String, // 'time_of_day', 'session_length', 'difficulty_preference'
    pub pattern_data: JsonValue,
    pub confidence_score: Option<f32>,
    pub detected_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    pub insight_type: String,
    pub title: String,
    pub description: String,
    pub data: JsonValue,
    pub confidence: f32,
    pub actionable: bool,
    pub suggestions: Vec<String>,
}

// ============== WebSocket & Real-time ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WsSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub connection_id: String,
    pub subscription_type: String, // 'recommendations', 'study_insights', 'progress_updates'
    pub active: bool,
    pub connected_at: DateTime<Utc>,
    pub last_ping_at: Option<DateTime<Utc>>,
    pub disconnected_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_type: String,
    pub payload: JsonValue,
    pub timestamp: DateTime<Utc>,
}

// ============== Spaced Repetition Algorithms ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacedRepetitionParams {
    pub algorithm: String, // 'sm2', 'leitner', 'exponential'
    pub ease_factor: f32,
    pub interval: i32,
    pub repetitions: i32,
    pub quality: i32, // 0-5 rating
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacedRepetitionResult {
    pub next_interval: i32,
    pub next_ease_factor: f32,
    pub next_review_date: DateTime<Utc>,
    pub difficulty_adjustment: f32,
}

// ============== Vertex AI Integration ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexAiRequest {
    pub prompt: String,
    pub model: String, // e.g., "text-bison", "gemini-pro"
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexAiResponse {
    pub text: String,
    pub tokens_used: i32,
    pub model: String,
    pub finish_reason: String,
}

// ============== User Learning Statistics (Materialized View) ==============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLearningStats {
    pub user_id: Uuid,
    pub unique_cards_studied: Option<i64>,
    pub total_study_events: Option<i64>,
    pub avg_response_time_ms: Option<i32>,
    pub total_correct: Option<i32>,
    pub total_incorrect: Option<i32>,
    pub accuracy_rate: Option<f32>,
    pub last_study_time: Option<DateTime<Utc>>,
    pub study_days: Option<i64>,
}

// ============== Study Session Enhancement ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStudySessionConfig {
    pub enable_ai_ordering: bool,
    pub difficulty_preference: Option<String>, // 'easy_first', 'hard_first', 'mixed', 'adaptive'
    pub focus_weak_cards: bool,
    pub include_overdue: bool,
    pub max_new_cards: Option<i32>,
    pub review_algorithm: String, // 'sm2', 'leitner', 'exponential'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyCardSuggestion {
    pub card_id: Uuid,
    pub reason: String,
    pub priority_score: f32,
    pub estimated_difficulty: f32,
    pub last_performance: Option<String>,
    pub suggested_time_seconds: Option<i32>,
}

// ============== Batch Processing ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAnalyticsEvent {
    pub events: Vec<CreateStudyEventDto>,
    pub batch_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

// ============== Error Responses ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiServiceError {
    pub error_type: String,
    pub message: String,
    pub details: Option<JsonValue>,
    pub retry_after: Option<i32>, // seconds
}
