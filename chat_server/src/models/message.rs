use super::Message;
use crate::{models::ChatFile, AppError, AppState};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub file: Vec<String>,
}

#[allow(dead_code)]
impl AppState {
    pub async fn create_message(
        &self,
        input: CreateMessage,
        chat_id: u64,
        user_id: u64,
    ) -> Result<Message, AppError> {
        let base_dir = &self.config.server.base_dir;
        if input.content.is_empty() {
            return Err(AppError::CreateMessageError(
                "Content can't be empty".to_string(),
            ));
        }
        for s in &input.file {
            let file = ChatFile::from_str(s)?;
            if !file.path(base_dir).exists() {
                return Err(AppError::CreateMessageError(format!(
                    "file {} doesn't exist",
                    s
                )));
            }
        }
        let message = sqlx::query_as(
            "
            INSERT INTO messages (chat_id, sender_id, content, files)
            VALUES ($1, $2, $3, $4)
            RETURNING id, chat_id, sender_id, content, files, created_at
            ",
        )
        .bind(chat_id as i64)
        .bind(user_id as i64)
        .bind(input.content)
        .bind(input.file)
        .fetch_one(&self.pool)
        .await?;
        Ok(message)
    }
}
