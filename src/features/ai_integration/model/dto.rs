use serde::{Deserialize, Serialize};
use async_graphql::{InputObject, SimpleObject};
use validator::Validate;
use utoipa::ToSchema;

use crate::entities::ai::ChatMessage;

#[derive(Debug, Deserialize, Validate, InputObject, ToSchema)]
pub struct ChatRequest {
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
    #[serde(default)]
    pub history: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Validate, InputObject, ToSchema)]
pub struct GenerateRequest {
    #[validate(length(min = 1, message = "Prompt cannot be empty"))]
    pub prompt: String,
    #[serde(default)]
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, SimpleObject, ToSchema)]
pub struct ChatResponse {
    pub response: String,
    pub model: String,
}

#[derive(Debug, Serialize, SimpleObject, ToSchema)]
pub struct GenerateResponse {
    pub text: String,
    pub model: String,
}
