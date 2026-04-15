use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{AppState, BackendError, handlers::helper::resolve_me_id};

#[serde_as]
#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    user_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    github_id: i64,
    handle: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

#[axum::debug_handler]
pub async fn get(Path(user_id): Path<i64>, State(state): State<AppState>) -> crate::BackendResult<Json<User>> {
    fetch_from_id(user_id, state.db_pool.clone()).await
}

#[axum::debug_handler]
pub async fn get_me(session: Session, State(state): State<AppState>) -> crate::BackendResult<Json<User>> {
    let id = resolve_me_id(&session).await?;
    fetch_from_id(id, state.db_pool.clone()).await
}

#[axum::debug_handler]
pub async fn resolve_handle_to_id(
    Path(handle): Path<String>,
    state: State<AppState>,
) -> crate::BackendResult<Json<i64>> {
    let result: Result<i64, _> = sqlx::query_scalar!(
        "SELECT user_id \
        FROM users \
        WHERE handle=$1",
        handle
    )
    .fetch_one(&state.db_pool)
    .await;
    match result {
        Ok(id) => Ok(Json(id)),
        Err(sqlx::Error::RowNotFound) => Err(BackendError::NotFound("User".to_string())),
        Err(e) => Err(e.into()),
    }
}

async fn fetch_from_id(user_id: i64, pool: PgPool) -> crate::BackendResult<Json<User>> {
    let result = sqlx::query_as!(
        User,
        "SELECT user_id, github_id, handle, created_at \
        FROM users \
        WHERE user_id=$1",
        user_id
    )
    .fetch_one(&pool)
    .await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(sqlx::Error::RowNotFound) => Err(BackendError::NotFound("User".to_string())),
        Err(e) => Err(e.into()),
    }
}
