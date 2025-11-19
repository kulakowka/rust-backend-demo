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
    ai_handler::{chat, chat_stream, generate},
    dto::{ChatRequest, ChatResponse, CreateUserRequest, GenerateRequest, GenerateResponse, UpdateUserRequest, UserResponse},
    handler::{create_user, delete_user, get_user, get_users, update_user},
    model::User,
    schema::{AppSchema, MutationRoot, QueryRoot},
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handler::get_users,
        crate::handler::get_user,
        crate::handler::create_user,
        crate::handler::update_user,
        crate::handler::delete_user,
        crate::ai_handler::chat,
        crate::ai_handler::generate,
        crate::ai_handler::chat_stream,
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
