use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::{
    config::VertexAiConfig,
    models::ai::{VertexAiRequest, VertexAiResponse},
};

// Google OAuth2 token
#[derive(Debug, Clone)]
struct AccessToken {
    token: String,
    expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

#[derive(Debug, Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
    safety_settings: Vec<SafetySetting>,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
    InlineData { inline_data: InlineData },
}

#[derive(Debug, Serialize)]
struct InlineData {
    mime_type: String,
    data: String, // Base64 encoded
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    temperature: f32,
    top_p: f32,
    top_k: i32,
    max_output_tokens: i32,
    stop_sequences: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SafetySetting {
    category: String,
    threshold: String,
}

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ContentResponse,
    finish_reason: String,
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
    role: String,
}

#[derive(Debug, Deserialize)]
struct PartResponse {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UsageMetadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
}

#[derive(Debug, Deserialize)]
struct SafetyRating {
    category: String,
    probability: String,
}

pub struct VertexAiClient {
    config: VertexAiConfig,
    http_client: Client,
    access_token: Option<AccessToken>,
}

impl VertexAiClient {
    pub fn new(config: VertexAiConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
            access_token: None,
        }
    }

    // Get or refresh access token
    async fn get_access_token(&mut self) -> Result<String> {
        // Check if we have a valid token
        if let Some(token) = &self.access_token {
            if token.expires_at > Utc::now() + Duration::seconds(60) {
                return Ok(token.token.clone());
            }
        }

        // Get new token
        let token = self.fetch_access_token().await?;
        self.access_token = Some(token.clone());
        Ok(token.token)
    }

    // Fetch access token from Google OAuth2
    async fn fetch_access_token(&self) -> Result<AccessToken> {
        // For production, use service account credentials
        // For now, we'll use the environment variable approach
        
        if let Some(cred_path) = &self.config.credentials_path {
            // Load service account JSON and create JWT
            // This is simplified - in production, use proper JWT signing
            info!("Loading credentials from: {}", cred_path);
            
            // Use Google's default authentication flow
            let token_url = "https://oauth2.googleapis.com/token";
            
            // Create JWT assertion (simplified)
            let response = self.http_client
                .post(token_url)
                .json(&json!({
                    "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
                    "assertion": self.create_jwt_assertion()?
                }))
                .send()
                .await?;

            if response.status().is_success() {
                let token_response: TokenResponse = response.json().await?;
                Ok(AccessToken {
                    token: token_response.access_token,
                    expires_at: Utc::now() + Duration::seconds(token_response.expires_in),
                })
            } else {
                error!("Failed to get access token: {}", response.status());
                Err(anyhow::anyhow!("Failed to authenticate with Google Cloud"))
            }
        } else {
            // Use Application Default Credentials (ADC)
            // This works in Google Cloud environments or with gcloud auth
            self.get_adc_token().await
        }
    }

    // Get token using Application Default Credentials
    async fn get_adc_token(&self) -> Result<AccessToken> {
        let metadata_url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
        
        let response = self.http_client
            .get(metadata_url)
            .header("Metadata-Flavor", "Google")
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let token_response: TokenResponse = resp.json().await?;
                Ok(AccessToken {
                    token: token_response.access_token,
                    expires_at: Utc::now() + Duration::seconds(token_response.expires_in),
                })
            }
            _ => {
                warn!("Failed to get ADC token, using mock token for development");
                // For development, return a mock token
                Ok(AccessToken {
                    token: "mock-development-token".to_string(),
                    expires_at: Utc::now() + Duration::hours(1),
                })
            }
        }
    }

    // Create JWT assertion for service account authentication
    fn create_jwt_assertion(&self) -> Result<String> {
        // This is a simplified version
        // In production, properly parse service account JSON and sign JWT
        Ok("mock-jwt-assertion".to_string())
    }

    // Generate content using Vertex AI
    pub async fn generate_content(&mut self, request: VertexAiRequest) -> Result<VertexAiResponse> {
        let access_token = self.get_access_token().await?;
        
        let api_url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            self.config.location,
            self.config.project_id,
            self.config.location,
            request.model.as_str()
        );

        let generate_request = GenerateContentRequest {
            contents: vec![Content {
                parts: vec![Part::Text { text: request.prompt }],
                role: "user".to_string(),
            }],
            generation_config: GenerationConfig {
                temperature: request.temperature.unwrap_or(self.config.temperature),
                top_p: request.top_p.unwrap_or(0.95),
                top_k: request.top_k.unwrap_or(40),
                max_output_tokens: request.max_tokens.unwrap_or(self.config.max_tokens),
                stop_sequences: vec![],
            },
            safety_settings: vec![
                SafetySetting {
                    category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
                },
                SafetySetting {
                    category: "HARM_CATEGORY_HARASSMENT".to_string(),
                    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
                },
            ],
        };

        let response = timeout(
            std::time::Duration::from_secs(self.config.timeout_seconds),
            self.http_client
                .post(&api_url)
                .header("Authorization", format!("Bearer {}", access_token))
                .header("Content-Type", "application/json")
                .json(&generate_request)
                .send()
        ).await??;

        if response.status().is_success() {
            let generate_response: GenerateContentResponse = response.json().await?;
            
            if let Some(candidate) = generate_response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    if let Some(text) = &part.text {
                        let tokens_used = generate_response.usage_metadata
                            .map(|u| u.total_token_count)
                            .unwrap_or(0);
                        
                        return Ok(VertexAiResponse {
                            text: text.clone(),
                            tokens_used,
                            model: request.model,
                            finish_reason: candidate.finish_reason.clone(),
                        });
                    }
                }
            }
            
            Err(anyhow::anyhow!("No valid response from Vertex AI"))
        } else {
            let error_text = response.text().await?;
            error!("Vertex AI API error: {}", error_text);
            Err(anyhow::anyhow!("Vertex AI API error: {}", error_text))
        }
    }

    // Generate flashcards from text content
    pub async fn generate_flashcards(
        &mut self,
        text: &str,
        options: &FlashcardGenerationOptions,
    ) -> Result<Vec<GeneratedFlashcard>> {
        let prompt = self.build_flashcard_prompt(text, options);
        
        let request = VertexAiRequest {
            prompt,
            model: self.config.default_model.clone(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
            top_p: Some(0.95),
            top_k: Some(40),
        };

        let response = self.generate_content(request).await?;
        self.parse_flashcards(&response.text)
    }

    // Build prompt for flashcard generation
    fn build_flashcard_prompt(&self, text: &str, options: &FlashcardGenerationOptions) -> String {
        let max_cards = options.max_cards.unwrap_or(10);
        let difficulty = options.difficulty.as_deref().unwrap_or("medium");
        let format = options.format.as_deref().unwrap_or("question_answer");
        
        format!(
            r#"Generate {} flashcards from the following text. 
            Difficulty level: {}
            Format: {}
            
            Requirements:
            1. Each flashcard should test understanding, not just memorization
            2. Include a mix of factual and conceptual questions
            3. Make the answers clear and concise
            4. If the text contains examples, use them in the flashcards
            
            Format the output as JSON array with objects containing:
            - "front": the question or prompt
            - "back": the answer
            - "explanation": optional additional context (only if helpful)
            - "difficulty": estimated difficulty (1-5)
            - "tags": relevant topic tags as array
            
            Text to process:
            {}
            
            Generate exactly {} flashcards as a valid JSON array:"#,
            max_cards, difficulty, format, text, max_cards
        )
    }

    // Parse flashcards from AI response
    fn parse_flashcards(&self, response: &str) -> Result<Vec<GeneratedFlashcard>> {
        // Try to extract JSON from the response
        let json_start = response.find('[').unwrap_or(0);
        let json_end = response.rfind(']').map(|i| i + 1).unwrap_or(response.len());
        let json_str = &response[json_start..json_end];
        
        match serde_json::from_str::<Vec<GeneratedFlashcard>>(json_str) {
            Ok(flashcards) => Ok(flashcards),
            Err(e) => {
                warn!("Failed to parse flashcards JSON: {}", e);
                // Try to extract flashcards manually as fallback
                self.parse_flashcards_fallback(response)
            }
        }
    }

    // Fallback parser for flashcards
    fn parse_flashcards_fallback(&self, response: &str) -> Result<Vec<GeneratedFlashcard>> {
        let mut flashcards = Vec::new();
        
        // Simple pattern matching for Q&A format
        let lines: Vec<&str> = response.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            if lines[i].contains("Front:") || lines[i].contains("Question:") {
                let front = lines[i]
                    .replace("Front:", "")
                    .replace("Question:", "")
                    .trim()
                    .to_string();
                
                if i + 1 < lines.len() && (lines[i + 1].contains("Back:") || lines[i + 1].contains("Answer:")) {
                    let back = lines[i + 1]
                        .replace("Back:", "")
                        .replace("Answer:", "")
                        .trim()
                        .to_string();
                    
                    flashcards.push(GeneratedFlashcard {
                        front,
                        back,
                        explanation: None,
                        difficulty: Some(3),
                        tags: vec![],
                    });
                    
                    i += 2;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        
        if flashcards.is_empty() {
            Err(anyhow::anyhow!("Could not parse any flashcards from response"))
        } else {
            Ok(flashcards)
        }
    }

    // Summarize document content
    pub async fn summarize_document(&mut self, text: &str, max_length: Option<i32>) -> Result<String> {
        let max_length = max_length.unwrap_or(500);
        
        let prompt = format!(
            r#"Provide a concise summary of the following text in approximately {} words.
            Focus on the main ideas, key concepts, and important details.
            
            Text:
            {}
            
            Summary:"#,
            max_length, text
        );

        let request = VertexAiRequest {
            prompt,
            model: self.config.default_model.clone(),
            max_tokens: Some(max_length * 2), // Approximate tokens
            temperature: Some(0.3), // Lower temperature for more focused summaries
            top_p: Some(0.9),
            top_k: Some(30),
        };

        let response = self.generate_content(request).await?;
        Ok(response.text)
    }

    // Extract key concepts from text
    pub async fn extract_concepts(&mut self, text: &str) -> Result<Vec<String>> {
        let prompt = format!(
            r#"Extract the key concepts, terms, and topics from the following text.
            List each concept on a new line, without numbering or bullets.
            Focus on important nouns, technical terms, and main ideas.
            
            Text:
            {}
            
            Key concepts:"#,
            text
        );

        let request = VertexAiRequest {
            prompt,
            model: self.config.default_model.clone(),
            max_tokens: Some(500),
            temperature: Some(0.2),
            top_p: Some(0.9),
            top_k: Some(20),
        };

        let response = self.generate_content(request).await?;
        
        Ok(response.text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect())
    }
}

// Helper structures for flashcard generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashcardGenerationOptions {
    pub max_cards: Option<i32>,
    pub difficulty: Option<String>,
    pub format: Option<String>,
    pub include_explanations: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFlashcard {
    pub front: String,
    pub back: String,
    pub explanation: Option<String>,
    pub difficulty: Option<i32>,
    pub tags: Vec<String>,
}
