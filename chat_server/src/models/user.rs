use super::User;
use crate::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::mem;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub fullname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

#[allow(unused)]
impl User {
    // Find a user by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as("SELECT id, fullname, email, created_at FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;
        Ok(user)
    }

    // Create a new user
    pub async fn create(input: &CreateUser, pool: &PgPool) -> Result<Self, AppError> {
        let password_hash = hash_password(&input.password)?;
        // check if email exists
        let user = Self::find_by_email(&input.email, pool).await?;
        if user.is_some() {
            return Err(AppError::EmailAlreadyExists(input.email.clone()));
        }
        let user = sqlx::query_as(
            "
            INSERT INTO users (email, fullname, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, fullname, email, created_at
            ",
        )
        .bind(&input.email)
        .bind(&input.fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    // Verify email and password
    pub async fn verify(input: &SigninUser, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, fullname, email, password_hash, created_at FROM users WHERE email = $1",
        )
        .bind(&input.email)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid =
                    verify_password(&input.password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();
    Ok(is_valid)
}

#[allow(unused)]
#[cfg(test)]
impl CreateUser {
    pub fn new(fullname: &str, email: &str, password: &str) -> Self {
        Self {
            fullname: fullname.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[allow(unused)]
#[cfg(test)]
impl SigninUser {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[allow(unused)]
#[cfg(test)]
impl User {
    pub fn new(id: i64, fullname: &str, email: &str) -> Self {
        use chrono::Utc;
        Self {
            id,
            fullname: fullname.to_string(),
            email: email.to_string(),
            password_hash: None,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    #[test]
    fn hash_password_and_verify_should_work() -> Result<()> {
        let password = "hunter42";
        let password_hash = hash_password(password)?;
        assert_eq!(password_hash.len(), 97);
        assert!(verify_password(password, &password_hash)?);
        Ok(())
    }
}

//     #[tokio::test]
//     async fn create_duplicate_user_should_fail() -> Result<()> {
//         let pool = PgPool::connect("postgres://postgres:@localhost/chat").await?;
//         let email = "Meng@123.com";
//         let fullname = "Team Meng";
//         let password = "hunter42";
//         let input = CreateUser::new(fullname, email, password);
//         User::create(&input, &pool).await?;
//         let ret = User::create(&input, &pool).await;
//         match ret {
//             Err(AppError::EmailAlreadyExists(email)) => {
//                 assert_eq!(email, input.email);
//             }
//             _ => (),
//         }
//         Ok(())
//     }
// }

//     #[tokio::test]
//     async fn create_and_verify_user_should_work() -> Result<()> {
//         let pool = PgPool::connect("postgres://postgres:@localhost/chat").await?;
//         let email = "Meng@123.com";
//         let fullname = "Team Meng";
//         let password = "hunter42";
//         let input = CreateUser::new(fullname, email, password);
//         let user = User::create(&input, &pool).await?;
//         assert_eq!(user.email, email);
//         assert_eq!(user.fullname, fullname);
//         assert!(user.id > 0);

//         let user = User::find_by_email(email, &pool).await?;
//         assert!(user.is_some());
//         let user = user.unwrap();
//         assert_eq!(user.email, email);
//         assert_eq!(user.fullname, fullname);

//         let input = SigninUser::new(&email, &password);
//         let user = User::verify(&input, &pool).await?;
//         assert!(user.is_some());
//         Ok(())
//     }
// }
