use axum::{
    extract::State,
    http::StatusCode,
    response::{sse::Event, IntoResponse, Response, Sse},
    Json,
};
use futures::stream::{self, Stream};
use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;

use crate::{
    dto::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse},
    error::AppError,
    state::AppState,
};

/// Chat with AI
#[utoipa::path(
    post,
    path = "/ai/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Successful chat response", body = ChatResponse),
        (status = 400, description = "Bad request"),
        (status = 502, description = "External service error")
    ),
    tag = "AI"
)]
pub async fn chat(
    State(state): State<AppState>,
    Json(input): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    let response = state.ai_service.chat(input).await?;
    Ok(Json(response))
}

/// Generate text from prompt
#[utoipa::path(
    post,
    path = "/ai/generate",
    request_body = GenerateRequest,
    responses(
        (status = 200, description = "Successful text generation", body = GenerateResponse),
        (status = 400, description = "Bad request"),
        (status = 502, description = "External service error")
    ),
    tag = "AI"
)]
pub async fn generate(
    State(state): State<AppState>,
    Json(input): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let response = state.ai_service.generate(input).await?;
    Ok(Json(response))
}

/// Chat with AI using Server-Sent Events for streaming
#[utoipa::path(
    post,
    path = "/ai/chat/stream",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Streaming chat response"),
        (status = 400, description = "Bad request"),
        (status = 502, description = "External service error")
    ),
    tag = "AI"
)]
pub async fn chat_stream(
    State(state): State<AppState>,
    Json(input): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // For now, we'll simulate streaming by getting the full response
    // and sending it in chunks. In a real implementation, you'd use
    // the Gemini streaming API.
    let response = state.ai_service.chat(input).await?;
    
    // Split response into words for streaming effect
    let words: Vec<String> = response
        .response
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let stream = stream::iter(words)
        .map(|word| Ok(Event::default().data(word)))
        .then(|event| async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            event
        });

    Ok(Sse::new(stream))
}
