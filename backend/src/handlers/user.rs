use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use sqlx::{PgPool, prelude::*};
use tower_sessions::Session;

use crate::AppState;

#[derive(FromRow, Serialize)]
pub struct User {
    user_id: i64,
    github_id: i64,
    display_name: String,
    created_at: time::OffsetDateTime,
}

type UserResponse = Result<Json<User>, (StatusCode, &'static str)>;

pub async fn get_user_from_url(Path(user_id): Path<i64>, State(state): State<AppState>) -> UserResponse {
    fetch_user_from_id(user_id, state.db_pool.clone()).await
}

pub async fn get_user_me(session: Session, State(state): State<AppState>) -> UserResponse {
    let Ok(user_id): Result<Option<i64>, _> = session.get("user_id").await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
    };

    match user_id {
        Some(id) => fetch_user_from_id(id, state.db_pool.clone()).await,
        None => Err((StatusCode::UNAUTHORIZED, "Not logged in")),
    }
}

async fn fetch_user_from_id(id: i64, pool: PgPool) -> UserResponse {
    let result = sqlx::query_as!(
        User,
        "SELECT user_id, github_id, display_name, created_at \
        FROM users \
        WHERE user_id=$1",
        id
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
