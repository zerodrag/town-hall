use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use crate::AppState;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Quest {
    quest_id: i64,
    poster_id: i64,
    title: String,
    description: String,
    created_at: time::OffsetDateTime,
}

pub async fn get_quest_from_id(
    Path(quest_id): Path<i64>,
    State(db): State<AppState>,
) -> Result<Json<Quest>, impl IntoResponse> {
    let result = sqlx::query_as::<_, Quest>(
        "SELECT quest_id, poster_id, title, description, created_at \
        FROM quests \
        WHERE quest_id=$1",
    )
    .bind(quest_id)
    .fetch_one(&db.db_pool)
    .await;
    match result {
        Ok(quest) => Ok(Json(quest)),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Quest ID not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}
