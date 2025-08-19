use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
    pub upload: UploadConfig,
    pub ai: AiConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub origin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadConfig {
    pub max_file_size: usize,
    pub allowed_file_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub collect_analytics: bool,
    pub vertex_ai: VertexAiConfig,
    pub content_generation: ContentGenerationConfig,
    pub recommendations: RecommendationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VertexAiConfig {
    pub project_id: String,
    pub location: String,
    pub credentials_path: Option<String>,
    pub default_model: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentGenerationConfig {
    pub max_cards_per_batch: i32,
    pub min_confidence_score: f32,
    pub supported_formats: Vec<String>,
    pub use_local_fallback: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecommendationConfig {
    pub min_events_for_recommendations: i32,
    pub recommendation_refresh_hours: i32,
    pub max_recommendations_per_user: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret-change-this".to_string()),
                expiration: env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .unwrap_or(86400),
            },
            cors: CorsConfig {
                origin: env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string()),
            },
            upload: UploadConfig {
                max_file_size: env::var("MAX_FILE_SIZE")
                    .unwrap_or_else(|_| "10485760".to_string())
                    .parse()
                    .unwrap_or(10485760),
                allowed_file_types: env::var("ALLOWED_FILE_TYPES")
                    .unwrap_or_else(|_| "csv,txt,pdf,docx,doc".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
            ai: AiConfig {
                enabled: env::var("AI_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                collect_analytics: env::var("AI_COLLECT_ANALYTICS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                vertex_ai: VertexAiConfig {
                    project_id: env::var("VERTEX_AI_PROJECT_ID")
                        .unwrap_or_else(|_| String::new()),
                    location: env::var("VERTEX_AI_LOCATION")
                        .unwrap_or_else(|_| "us-central1".to_string()),
                    credentials_path: env::var("GOOGLE_APPLICATION_CREDENTIALS").ok(),
                    default_model: env::var("VERTEX_AI_MODEL")
                        .unwrap_or_else(|_| "gemini-pro".to_string()),
                    max_tokens: env::var("VERTEX_AI_MAX_TOKENS")
                        .unwrap_or_else(|_| "2048".to_string())
                        .parse()
                        .unwrap_or(2048),
                    temperature: env::var("VERTEX_AI_TEMPERATURE")
                        .unwrap_or_else(|_| "0.7".to_string())
                        .parse()
                        .unwrap_or(0.7),
                    timeout_seconds: env::var("VERTEX_AI_TIMEOUT")
                        .unwrap_or_else(|_| "30".to_string())
                        .parse()
                        .unwrap_or(30),
                },
                content_generation: ContentGenerationConfig {
                    max_cards_per_batch: env::var("AI_MAX_CARDS_PER_BATCH")
                        .unwrap_or_else(|_| "50".to_string())
                        .parse()
                        .unwrap_or(50),
                    min_confidence_score: env::var("AI_MIN_CONFIDENCE")
                        .unwrap_or_else(|_| "0.7".to_string())
                        .parse()
                        .unwrap_or(0.7),
                    supported_formats: env::var("AI_SUPPORTED_FORMATS")
                        .unwrap_or_else(|_| "pdf,docx,txt,csv,doc".to_string())
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect(),
                    use_local_fallback: env::var("AI_USE_LOCAL_FALLBACK")
                        .unwrap_or_else(|_| "false".to_string())
                        .parse()
                        .unwrap_or(false),
                },
                recommendations: RecommendationConfig {
                    min_events_for_recommendations: env::var("AI_MIN_EVENTS")
                        .unwrap_or_else(|_| "10".to_string())
                        .parse()
                        .unwrap_or(10),
                    recommendation_refresh_hours: env::var("AI_REFRESH_HOURS")
                        .unwrap_or_else(|_| "24".to_string())
                        .parse()
                        .unwrap_or(24),
                    max_recommendations_per_user: env::var("AI_MAX_RECOMMENDATIONS")
                        .unwrap_or_else(|_| "10".to_string())
                        .parse()
                        .unwrap_or(10),
                },
            },
        })
    }

    pub fn get_bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
