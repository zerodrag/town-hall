use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::query_as;
use tower_sessions::Session;
use validator::{Validate, ValidationError};

use crate::{
    AppState,
    handlers::helper::{self, SimpResp},
};

#[serde_as]
#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    #[serde_as(as = "DisplayFromStr")]
    quest_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    poster_id: i64,
    title: String,
    description: String,
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

#[derive(specta::Type, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
}

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let cleaned_title = title.trim();
    if !(1..=100).contains(&cleaned_title.len()) {
        return Err(ValidationError::new("Title must be 1 to 100 characters long"));
    }
    if cleaned_title == "Never gonna give you up" {
        return Err(ValidationError::new("Title must not be a rick roll"));
    }
    Ok(())
}

#[axum::debug_handler]
pub async fn create(
    session: Session,
    State(state): State<AppState>,
    Valid(Json(req)): Valid<Json<CreateQuestRequest>>,
) -> SimpResp<Json<i64>> {
    let trimmed_title = req.title.trim();

    let id = helper::resolve_current_user_id(&session).await?;
    let result: Result<i64, _> = sqlx::query_scalar!(
        "INSERT INTO quests (poster_id, title, description) \
        VALUES ($1, $2, $3) \
        RETURNING quest_id",
        id,
        trimmed_title,
        "",
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
