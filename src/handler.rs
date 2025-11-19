use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{CreateUserRequest, UpdateUserRequest, UserResponse},
    error::AppError,
    service::UserService,
};

#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "List all users", body = Vec<UserResponse>)
    )
)]
pub async fn get_users(
    State(service): State<UserService>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = service.get_users().await?;

    let response = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
            created_at: u.created_at.to_rfc3339(),
            updated_at: u.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User database id")
    ),
    responses(
        (status = 200, description = "Get user by id", body = UserResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service.get_user(id).await?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
        updated_at: user.updated_at.to_rfc3339(),
    }))
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "Create a new user", body = UserResponse),
        (status = 400, description = "Validation error")
    )
)]
pub async fn create_user(
    State(service): State<UserService>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(e.to_string()));
    }

    let user = service.create_user(payload).await?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
        updated_at: user.updated_at.to_rfc3339(),
    }))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User database id")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "Update user", body = UserResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn update_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::Validation(e.to_string()));
    }

    let updated_user = service.update_user(id, payload).await?;

    Ok(Json(UserResponse {
        id: updated_user.id.to_string(),
        name: updated_user.name,
        email: updated_user.email,
        created_at: updated_user.created_at.to_rfc3339(),
        updated_at: updated_user.updated_at.to_rfc3339(),
    }))
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = Uuid, Path, description = "User database id")
    ),
    responses(
        (status = 200, description = "Delete user"),
        (status = 404, description = "User not found")
    )
)]
pub async fn delete_user(
    State(service): State<UserService>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    service.delete_user(id).await?;

    Ok(Json(serde_json::json!({ "message": "User deleted" })))
}
