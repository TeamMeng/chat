use crate::models::User;
use axum::{response::IntoResponse, Extension};
use tracing::info;

pub async fn list_chat_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    info!("user: {:?}", user);
    "chat"
}

pub async fn create_chat_handler() -> impl IntoResponse {
    "create chat"
}

pub async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat"
}

pub async fn update_chat_handler() -> impl IntoResponse {
    "update chat"
}
