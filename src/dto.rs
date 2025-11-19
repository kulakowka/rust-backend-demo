use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use validator::Validate;

use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, InputObject, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, InputObject, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

// AI DTOs
use crate::ai_model::ChatMessage;
use async_graphql::SimpleObject;

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
