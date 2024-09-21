mod config;
mod error;
mod handlers;
mod middlewares;
mod models;
mod postgres;
mod utils;

use anyhow::Context;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
pub use config::AppConfig;
pub use error::{AppError, ErrorOutput};
use handlers::*;
use middlewares::{set_layer, verify_token};
pub use postgres::TestPg;
use sqlx::PgPool;
use std::{fmt::Debug, ops::Deref, sync::Arc};
use tokio::fs;
use utils::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;

    let api = Router::new()
        .route("/users", get(list_chat_users_handler))
        .route("/chats", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chats/:id",
            get(get_chat_handler)
                .patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chats/:id/messages", get(list_message_handler))
        .route("/upload", post(upload_handler))
        .route("/files/:ws_id/*path", get(file_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        // routes doesn't need token verification
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);

    Ok(set_layer(app))
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        fs::create_dir_all(&config.server.base_dir)
            .await
            .context("create base_dir failed")?;
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load ek failed")?;
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                ek,
                pool,
            }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
impl AppState {
    pub async fn new_for_test(config: AppConfig) -> Result<(TestPg, Self), AppError> {
        let dk = DecodingKey::load(&config.auth.pk).context("load pk failed")?;
        let ek = EncodingKey::load(&config.auth.sk).context("load ek failed")?;
        let post = config.server.db_url.rfind('/').expect("invalid db_url");
        let server_url = &config.server.db_url[..post];
        let (tdb, pool) = get_test_pool(Some(server_url)).await?;
        let state = Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        };
        Ok((tdb, state))
    }
}

#[cfg(test)]
pub async fn get_test_pool(url: Option<&str>) -> anyhow::Result<(TestPg, PgPool)> {
    use sqlx::Executor;
    let url = match url {
        Some(url) => url.to_string(),
        None => "postgres://postgres:postgres@localhost:5432".to_string(),
    };
    let tdb = TestPg::new(url, std::path::Path::new("../migrations"));
    let pool = tdb.get_pool().await;
    // run prepared sql to insert data dat
    let sql = include_str!("../fixtures/test.sql").split(';');
    let mut ts = pool.begin().await.expect("begin transaction failed");
    for s in sql {
        if !s.trim().is_empty() {
            ts.execute(s).await.expect("execute sql failed");
        }
    }
    ts.commit().await.expect("commit transaction failed");
    Ok((tdb, pool))
}
