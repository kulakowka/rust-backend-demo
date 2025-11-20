// Feature-Sliced Design Architecture
mod shared;
mod entities;
mod features;
mod app;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::shared::config::Config;
use crate::shared::database::create_pool;
use crate::features::user_management::infrastructure::PostgresUserRepository;
use crate::features::user_management::domain::UserService;
use crate::features::ai_integration::infrastructure::GeminiRepository;
use crate::features::ai_integration::domain::AIService;
use crate::app::{AppState, create_router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database pool and run migrations
    let pool = create_pool(&config.database_url).await?;

    // Initialize repositories
    let user_repository = std::sync::Arc::new(PostgresUserRepository::new(pool));
    let ai_repository = std::sync::Arc::new(GeminiRepository::new(config.gemini_api_key.clone()));

    // Initialize services
    let user_service = UserService::new(user_repository);
    let ai_service = AIService::new(ai_repository);

    // Create app state and router
    let state = AppState::new(user_service, ai_service);
    let app = create_router(state);

    // Start server
    let addr_str = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr_str).await?;
    tracing::info!("listening on {}", addr_str);
    axum::serve(listener, app).await?;

    Ok(())
}
