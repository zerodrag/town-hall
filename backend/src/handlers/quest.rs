use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::query_as;
use tower_sessions::Session;

use crate::AppState;

#[serde_as]
#[derive(Serialize, specta::Type)]
pub struct Quest {
    #[serde_as(as = "DisplayFromStr")]
    quest_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    poster_id: i64,
    title: String,
    description: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

type QuestResponse = Result<Json<Quest>, (StatusCode, &'static str)>;

pub async fn get_from_url(Path(quest_id): Path<i64>, State(state): State<AppState>) -> QuestResponse {
    let result = query_as!(
        Quest,
        "SELECT quest_id, poster_id, title, description, created_at \
        FROM quests \
        WHERE quest_id=$1",
        quest_id
    )
    .fetch_one(&state.db_pool)
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

#[derive(Deserialize, specta::Type)]
pub struct CreateQuestRequest {
    title: String,
    description: String,
}

pub async fn create(
    session: Session,
    State(state): State<AppState>,
    Json(request): Json<CreateQuestRequest>,
) -> Result<Json<i64>, (StatusCode, &'static str)> {
    let Ok(user_id): Result<Option<i64>, _> = session.get("user_id").await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
    };
    match user_id {
        Some(id) => {
            let result: Result<i64, _> = sqlx::query_scalar!(
                "INSERT INTO quests (poster_id, title, description) \
                VALUES ($1, $2, $3) \
                RETURNING quest_id",
                id,
                request.title,
                request.description
            )
            .fetch_one(&state.db_pool)
            .await;
            match result {
                Ok(quest_id) => Ok(Json(quest_id)),
                Err(e) => {
                    tracing::error!("Database Error: {e}");
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
                }
            }
        }
        None => Err((StatusCode::UNAUTHORIZED, "Not logged in")),
    }
}
