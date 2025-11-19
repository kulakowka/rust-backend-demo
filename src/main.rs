mod config;
mod dto;
mod error;
mod handler;
mod model;
mod repository;
mod route;
mod schema;
mod service;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Connecting to database at {}", config.database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    use std::sync::Arc;
    use crate::repository::PostgresUserRepository;
    use crate::service::UserService;

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let repository = Arc::new(PostgresUserRepository::new(pool));
    let service = UserService::new(repository);

    let app = route::create_router(service);

    let addr_str = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr_str).await?;
    tracing::info!("listening on {}", addr_str);
    axum::serve(listener, app).await?;

    Ok(())
}

