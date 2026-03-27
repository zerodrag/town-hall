use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::AppState;

#[serde_as]
#[derive(Serialize, specta::Type)]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    user_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    github_id: i64,
    handle: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

type UserResponse = Result<Json<User>, (StatusCode, &'static str)>;

pub async fn get_from_url(Path(user_id): Path<i64>, State(state): State<AppState>) -> UserResponse {
    fetch_from_id(user_id, state.db_pool.clone()).await
}

pub async fn get_me(session: Session, State(state): State<AppState>) -> UserResponse {
    let Ok(user_id): Result<Option<i64>, _> = session.get("user_id").await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
    };

    match user_id {
        Some(id) => fetch_from_id(id, state.db_pool.clone()).await,
        None => Err((StatusCode::UNAUTHORIZED, "Not logged in")),
    }
}

async fn fetch_from_id(user_id: i64, pool: PgPool) -> UserResponse {
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
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "User ID not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

pub async fn resolve_handle_to_id(
    Path(handle): Path<String>,
    state: State<AppState>,
) -> Result<Json<i64>, (StatusCode, &'static str)> {
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
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Handle not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database Error"))
        }
    }
}
