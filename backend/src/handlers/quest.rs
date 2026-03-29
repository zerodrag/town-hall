use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use sqlx::query_as;
use tower_sessions::Session;

use crate::{
    AppState,
    handlers::helper::{self, SimpResp},
};

#[serde_as]
#[derive(Serialize, specta::Type)]
pub struct Quest {
    #[serde_as(as = "DisplayFromStr")]
    quest_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    poster_id: i64,
    title: String,
    description: Option<String>,
    status: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

#[axum::debug_handler]
pub async fn get_from_url(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
) -> SimpResp<Json<Quest>> {
    let result = query_as!(
        Quest,
        "SELECT quest_id, poster_id, title, description, status, created_at \
        FROM quests \
        WHERE quest_id=$1",
        quest_id
    )
    .fetch_one(&state.db_pool)
    .await;
    match result {
        Ok(quest) => {
            if quest.status != "draft" {
                Ok(Json(quest))
            } else {
                if let Ok(id) = helper::resolve_current_user_id(&session).await
                    && id == quest.poster_id
                {
                    Ok(Json(quest))
                } else {
                    Err((StatusCode::NOT_FOUND, "Quest ID not found"))
                }
            }
        }
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Quest ID not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn create(session: Session, State(state): State<AppState>, Json(title): Json<String>) -> SimpResp<Json<i64>> {
    let cleaned_title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    if !(10..=100).contains(&cleaned_title.len()) {
        return Err((StatusCode::BAD_REQUEST, "Title must be between 10 to 100 characters"));
    }

    // TESTING ONLY. DELETE IN PROD
    if cleaned_title == "Never gonna give you up" {
        return Err((StatusCode::BAD_REQUEST, "Title must not be a rick roll"));
    }

    let id = helper::resolve_current_user_id(&session).await?;
    let result: Result<i64, _> = sqlx::query_scalar!(
        "INSERT INTO quests (poster_id, title) \
        VALUES ($1, $2) \
        RETURNING quest_id",
        id,
        cleaned_title
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
