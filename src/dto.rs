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
