use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use crate::Database;

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    user_id: i32,
    github_id: i32,
    display_name: String,
    created_at: time::OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    github_id: i32,
    display_name: String,
}

pub async fn get_user_from_id(
    Path(user_id): Path<u32>,
    State(db): State<Arc<Database>>,
) -> Result<Json<User>, impl IntoResponse> {
    let Ok(user_id_normalized): Result<i32, _> = user_id.try_into() else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid ID: User ID must be a signed 32-bit integer",
        ));
    };
    let result = sqlx::query_as::<_, User>(
        "SELECT user_id, github_id, display_name, created_at \
        FROM users \
        WHERE user_id=$1",
    )
    .bind(user_id_normalized)
    .fetch_one(&db.pool)
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

pub async fn create_user(
    State(db): State<Arc<Database>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, impl IntoResponse> {
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (github_id, display_name) \
        VALUES ($1, $2) \
        RETURNING user_id, github_id, display_name, created_at",
    )
    .bind(req.github_id)
    .bind(req.display_name)
    .fetch_one(&db.pool)
    .await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            tracing::error!("Database error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}
