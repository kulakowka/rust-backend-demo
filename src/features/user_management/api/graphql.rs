use async_graphql::{Context, EmptySubscription, Object, Schema};
use uuid::Uuid;
use validator::Validate;

use crate::{
    features::user_management::model::{CreateUserRequest, UpdateUserRequest},
    features::user_management::domain::UserService,
    features::ai_integration::model::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse},
    features::ai_integration::domain::AIService,
    entities::user::User,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let service = ctx.data::<UserService>()?;
        let users = service
            .get_users()
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(users)
    }

    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Option<User>> {
        let service = ctx.data::<UserService>()?;
        match service.get_user(id).await {
            Ok(user) => Ok(Some(user)),
            Err(crate::shared::error::AppError::NotFound) => Ok(None),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserRequest,
    ) -> async_graphql::Result<User> {
        if let Err(e) = input.validate() {
            return Err(async_graphql::Error::new(e.to_string()));
        }

        let service = ctx.data::<UserService>()?;
        let user = service
            .create_user(input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(user)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateUserRequest,
    ) -> async_graphql::Result<User> {
        if let Err(e) = input.validate() {
            return Err(async_graphql::Error::new(e.to_string()));
        }

        let service = ctx.data::<UserService>()?;
        let user = service
            .update_user(id, input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<bool> {
        let service = ctx.data::<UserService>()?;
        match service.delete_user(id).await {
            Ok(_) => Ok(true),
            Err(crate::shared::error::AppError::NotFound) => Ok(false),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }

    /// Chat with AI
    async fn chat(
        &self,
        ctx: &Context<'_>,
        input: ChatRequest,
    ) -> async_graphql::Result<ChatResponse> {
        if let Err(e) = input.validate() {
            return Err(async_graphql::Error::new(e.to_string()));
        }

        let service = ctx.data::<AIService>()?;
        let response = service
            .chat(input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(response)
    }

    /// Generate text from prompt
    async fn generate(
        &self,
        ctx: &Context<'_>,
        input: GenerateRequest,
    ) -> async_graphql::Result<GenerateResponse> {
        if let Err(e) = input.validate() {
            return Err(async_graphql::Error::new(e.to_string()));
        }

        let service = ctx.data::<AIService>()?;
        let response = service
            .generate(input)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(response)
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
