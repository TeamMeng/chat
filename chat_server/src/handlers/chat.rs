use axum::response::IntoResponse;
use tracing::instrument;

#[instrument]
pub async fn list_chat_handler() -> impl IntoResponse {
    "chat"
}

#[instrument]
pub async fn create_chat_handler() -> impl IntoResponse {
    "create chat"
}

#[instrument]
pub async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat"
}

#[instrument]
pub async fn update_chat_handler() -> impl IntoResponse {
    "update chat"
}
