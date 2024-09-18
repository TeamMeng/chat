mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub use user::{CreateUser, SigninUser};

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq, Clone)]
pub struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}
