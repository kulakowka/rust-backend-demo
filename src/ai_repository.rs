use async_trait::async_trait;
use google_generative_ai_rs::v1::api::Client;
use google_generative_ai_rs::v1::gemini::{request::Request, Content, Part, Role};
use std::sync::Arc;

use crate::ai_model::ChatMessage;
use crate::error::AppError;

#[async_trait]
pub trait AIRepository: Send + Sync {
    async fn chat(&self, message: String, history: Vec<ChatMessage>) -> Result<String, AppError>;
    async fn generate(&self, prompt: String) -> Result<String, AppError>;
}

pub struct GeminiRepository {
    client: Arc<Client>,
}

impl GeminiRepository {
    pub fn new(api_key: String) -> Self {
        let client = Client::new(api_key);
        Self {
            client: Arc::new(client),
        }
    }

    fn convert_history(&self, history: Vec<ChatMessage>) -> Vec<Content> {
        history
            .into_iter()
            .map(|msg| {
                let role = if msg.role == "user" {
                    Role::User
                } else {
                    Role::Model
                };
                Content {
                    role,
                    parts: vec![Part {
                        text: Some(msg.content),
                        inline_data: None,
                        file_data: None,
                        video_metadata: None,
                    }],
                }
            })
            .collect()
    }
}

#[async_trait]
impl AIRepository for GeminiRepository {
    async fn chat(&self, message: String, history: Vec<ChatMessage>) -> Result<String, AppError> {
        let mut contents = self.convert_history(history);
        
        // Add the current user message
        contents.push(Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(message),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        });

        let request = Request {
            contents,
            tools: vec![],
            safety_settings: vec![],
            generation_config: None,
        };

        let result = self.client
            .post(30, &request)
            .await
            .map_err(|e| AppError::ExternalService(format!("Gemini API error: {}", e)))?;

        // Extract text from response
        let text = result
            .rest()
            .ok_or_else(|| AppError::ExternalService("No response from Gemini".to_string()))?
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .ok_or_else(|| AppError::ExternalService("No response text from Gemini".to_string()))?;;

        Ok(text)
    }

    async fn generate(&self, prompt: String) -> Result<String, AppError> {
        let contents = vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(prompt),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }];

        let request = Request {
            contents,
            tools: vec![],
            safety_settings: vec![],
            generation_config: None,
        };

        let result = self.client
            .post(30, &request)
            .await
            .map_err(|e| AppError::ExternalService(format!("Gemini API error: {}", e)))?;

        let text = result
            .rest()
            .ok_or_else(|| AppError::ExternalService("No response from Gemini".to_string()))?
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
            .ok_or_else(|| AppError::ExternalService("No response text from Gemini".to_string()))?;

        Ok(text)
    }
}
