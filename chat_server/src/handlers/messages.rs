use axum::response::IntoResponse;
use tracing::instrument;

#[instrument]
pub async fn send_message_handler() -> impl IntoResponse {
    "send message"
}

#[instrument]
pub async fn list_message_handler() -> impl IntoResponse {
    "list message"
}
