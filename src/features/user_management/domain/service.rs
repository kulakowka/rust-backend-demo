use std::sync::Arc;
use uuid::Uuid;

use crate::{
    features::user_management::model::{CreateUserRequest, UpdateUserRequest},
    features::user_management::infrastructure::UserRepository,
    shared::error::AppError,
    entities::user::User,
};

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        self.repository
            .find_all()
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, AppError> {
        self.repository
            .find_by_id(id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or(AppError::NotFound)
    }

    pub async fn create_user(&self, input: CreateUserRequest) -> Result<User, AppError> {
        self.repository
            .create(input.name, input.email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        input: UpdateUserRequest,
    ) -> Result<User, AppError> {
        let user = self.get_user(id).await?;

        let name = input.name.unwrap_or(user.name);
        let email = input.email.unwrap_or(user.email);

        self.repository
            .update(id, name, email)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let deleted = self
            .repository
            .delete(id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if !deleted {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
