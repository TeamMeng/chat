use axum::response::IntoResponse;

pub async fn send_message_handler() -> impl IntoResponse {
    "send message"
}

pub async fn list_message_handler() -> impl IntoResponse {
    "list message"
}
