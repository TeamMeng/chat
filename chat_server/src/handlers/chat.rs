use crate::{
    models::{CreateChat, User},
    AppError, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

pub async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chats = state.fetch_chats(user.ws_id as _).await?;
    Ok((StatusCode::OK, Json(chats)))
}

pub async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.chat_create(input, user.ws_id as _).await?;
    Ok((StatusCode::CREATED, Json(chat)))
}

pub async fn get_chat_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let chat = state.get_chat_by_id(id).await?;
    match chat {
        Some(chat) => Ok((StatusCode::OK, Json(chat))),
        None => Err(AppError::NotFound(format!("chat id {}", id))),
    }
}

pub async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat"
}

pub async fn update_chat_handler() -> impl IntoResponse {
    "update chat"
}
