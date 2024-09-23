mod chat;
mod file;
mod message;
mod user;
mod workspace;

pub use chat::CreateChat;
pub use message::{CreateMessage, ListMessage};
use serde::{Deserialize, Serialize};
pub use user::{CreateUser, SigninUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String,
    pub hash: String,
}
