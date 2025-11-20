use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    features::user_management::api::{
        create_user, delete_user, get_user, get_users, update_user,
        MutationRoot, QueryRoot, AppSchema,
    },
    features::user_management::model::{CreateUserRequest, UpdateUserRequest, UserResponse},
    features::ai_integration::api::{chat, chat_stream, generate},
    features::ai_integration::model::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse},
    entities::user::User,
    app::state::AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::user_management::api::rest::get_users,
        crate::features::user_management::api::rest::get_user,
        crate::features::user_management::api::rest::create_user,
        crate::features::user_management::api::rest::update_user,
        crate::features::user_management::api::rest::delete_user,
        crate::features::ai_integration::api::rest::chat,
        crate::features::ai_integration::api::rest::generate,
        crate::features::ai_integration::api::rest::chat_stream,
    ),
    components(
        schemas(User, CreateUserRequest, UpdateUserRequest, UserResponse, ChatRequest, ChatResponse, GenerateRequest, GenerateResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "AI", description = "AI-powered endpoints using Gemini")
    )
)]
struct ApiDoc;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn create_router(state: AppState) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state.user_service.clone())
        .data(state.ai_service.clone())
        .finish();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/ai/chat", post(chat))
        .route("/ai/generate", post(generate))
        .route("/ai/chat/stream", post(chat_stream))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
        .with_state(state)
}
