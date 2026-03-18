use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use crate::AppState;

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    user_id: i64,
    github_id: i64,
    display_name: String,
    email: String,
    created_at: time::OffsetDateTime,
}

pub async fn get_user_from_id(
    Path(user_id): Path<i64>,
    State(db): State<Arc<AppState>>,
) -> Result<Json<User>, impl IntoResponse> {
    let result = sqlx::query_as::<_, User>(
        "SELECT user_id, github_id, display_name, created_at \
        FROM users \
        WHERE user_id=$1",
    )
    .bind(user_id)
    .fetch_one(&db.db_pool)
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
