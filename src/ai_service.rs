use std::sync::Arc;
use validator::Validate;

use crate::{
    ai_model::ChatMessage,
    ai_repository::AIRepository,
    dto::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse},
    error::AppError,
};

#[derive(Clone)]
pub struct AIService {
    repository: Arc<dyn AIRepository>,
}

impl AIService {
    pub fn new(repository: Arc<dyn AIRepository>) -> Self {
        Self { repository }
    }

    pub async fn chat(&self, input: ChatRequest) -> Result<ChatResponse, AppError> {
        // Validate input
        input
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Call repository
        let response = self
            .repository
            .chat(input.message, input.history)
            .await?;

        Ok(ChatResponse {
            response,
            model: "gemini-2.0-flash-exp".to_string(),
        })
    }

    pub async fn generate(&self, input: GenerateRequest) -> Result<GenerateResponse, AppError> {
        // Validate input
        input
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Call repository
        let text = self.repository.generate(input.prompt).await?;

        Ok(GenerateResponse {
            text,
            model: "gemini-2.0-flash-exp".to_string(),
        })
    }
}
