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

#[derive(FromRow, Serialize, Deserialize)]
pub struct Quest {
    quest_id: i32,
    poster_id: i32,
    title: String,
    description: String,
    created_at: time::OffsetDateTime,
}

pub async fn get_quest_from_id(
    Path(quest_id): Path<u32>,
    State(db): State<Arc<Database>>,
) -> Result<Json<Quest>, impl IntoResponse> {
    let Ok(quest_id_normalized): Result<i32, _> = quest_id.try_into() else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid ID: Quest ID must be a signed 32-bit integer",
        ));
    };
    let result = sqlx::query_as::<_, Quest>(
        "SELECT quest_id, poster_id, title, description, created_at \
        FROM quests \
        WHERE quest_id=$1",
    )
    .bind(quest_id_normalized)
    .fetch_one(&db.pool)
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
