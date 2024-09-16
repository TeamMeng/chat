mod auth;
mod chat;
mod messages;

use axum::response::IntoResponse;
use tracing::instrument;

pub(crate) use auth::*;
pub(crate) use chat::*;
pub(crate) use messages::*;

#[instrument]
pub(crate) async fn index_handler() -> impl IntoResponse {
    "Hello World"
}
