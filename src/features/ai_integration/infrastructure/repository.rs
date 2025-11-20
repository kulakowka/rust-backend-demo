use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::entities::ai::ChatMessage;
use crate::shared::error::AppError;

#[async_trait]
pub trait AIRepository: Send + Sync {
    async fn chat(&self, message: String, history: Vec<ChatMessage>) -> Result<String, AppError>;
    async fn generate(&self, prompt: String) -> Result<String, AppError>;
}

pub struct GeminiRepository {
    client: Arc<Client>,
    api_key: String,
    model: String,
}

// Gemini API request/response structures
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContentResponse,
}

#[derive(Debug, Deserialize)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

#[derive(Debug, Deserialize)]
struct GeminiPartResponse {
    text: String,
}

impl GeminiRepository {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        // Use latest Gemini model - gemini-2.0-flash-exp, gemini-pro, or custom
        let model = std::env::var("GEMINI_MODEL")
            .unwrap_or_else(|_| "gemini-2.0-flash-exp".to_string());
        
        Self {
            client: Arc::new(client),
            api_key,
            model,
        }
    }

    fn convert_history(&self, history: Vec<ChatMessage>) -> Vec<GeminiContent> {
        history
            .into_iter()
            .map(|msg| {
                // Gemini API only accepts "user" and "model" roles
                // Convert "assistant" to "model" for compatibility
                let role = if msg.role == "user" {
                    "user".to_string()
                } else {
                    "model".to_string()
                };
                
                GeminiContent {
                    role,
                    parts: vec![GeminiPart { text: msg.content }],
                }
            })
            .collect()
    }

    async fn call_gemini_api(&self, contents: Vec<GeminiContent>) -> Result<String, AppError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let request_body = GeminiRequest { contents };

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to call Gemini API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalService(format!(
                "Gemini API error ({}): {}",
                status, error_text
            )));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse Gemini response: {}", e)))?;

        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| AppError::ExternalService("No response from Gemini".to_string()))?;

        Ok(text)
    }
}

#[async_trait]
impl AIRepository for GeminiRepository {
    async fn chat(&self, message: String, history: Vec<ChatMessage>) -> Result<String, AppError> {
        let mut contents = self.convert_history(history);
        
        // Add the current user message
        contents.push(GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart { text: message }],
        });

        self.call_gemini_api(contents).await
    }

    async fn generate(&self, prompt: String) -> Result<String, AppError> {
        let contents = vec![GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart { text: prompt }],
        }];

        self.call_gemini_api(contents).await
    }
}
