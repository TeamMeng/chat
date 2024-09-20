use crate::{
    models::{User, Workspace},
    AppError, AppState,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};

pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = Workspace::fetch_all_chat_users(user.ws_id as _, &state.pool).await?;
    Ok(Json(users))
}
