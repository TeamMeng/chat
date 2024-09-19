use crate::{
    models::{CreateUser, SigninUser, User},
    AppError, AppState, ErrorOutput,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AuthOutput {
    token: String,
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&input, &state.pool).await?;
    let token = state.ek.sign(user)?;
    // let mut header = HeaderMap::new();
    // header.insert("X-Token", HeaderValue::from_str(&token)?);
    // Ok((StatusCode::CREATED, header))
    let body = Json(AuthOutput { token });
    Ok((StatusCode::CREATED, body))
}

pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::verify(&input, &state.pool).await?;
    match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            Ok((StatusCode::OK, Json(AuthOutput { token })).into_response())
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid email or password"));
            Ok((StatusCode::FORBIDDEN, body).into_response())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppConfig;
    use anyhow::Result;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let state = AppState::try_new(config).await?;
        let input = CreateUser::new("Signin Meng", "Signin@123.com", "hunter42");
        let ret = signup_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::CREATED);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }

    #[tokio::test]
    async fn signup_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let state = AppState::try_new(config).await?;
        let email = "Signup@123.com";
        let password = "hunter42";
        let input = CreateUser::new("Signup Meng", email, password);
        User::create(&input, &state.pool).await?;
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::OK);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }

    #[tokio::test]
    async fn signup_duplicate_should_409() -> Result<()> {
        let config = AppConfig::load()?;
        let state = AppState::try_new(config).await?;
        let email = "Signup-failed@123.com";
        let password = "hunter42";
        let input = CreateUser::new("Signup Meng", email, password);
        signup_handler(State(state.clone()), Json(input.clone())).await?;

        let ret = signup_handler(State(state), Json(input))
            .await
            .into_response();

        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "email already exists: Signup-failed@123.com");

        Ok(())
    }

    #[tokio::test]
    async fn signup_with_non_exist_user_should_403() -> Result<()> {
        let config = AppConfig::load()?;
        let state = AppState::try_new(config).await?;
        let email = "non-exist@123.com";
        let password = "hunter42";
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await
            .into_response();
        assert_eq!(ret.status(), StatusCode::FORBIDDEN);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "Invalid email or password");
        Ok(())
    }
}
