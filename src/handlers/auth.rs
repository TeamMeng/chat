use axum::response::IntoResponse;
use tracing::instrument;

#[instrument]
pub async fn signin_handler() -> impl IntoResponse {
    "signin"
}

#[instrument]
pub async fn signup_handler() -> impl IntoResponse {
    "signin"
}
