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

use crate::{AppState, BackendError, BackendResult, handlers::helper};

#[derive(PartialEq, Debug, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(type_name = "quest_status", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum QuestStatus {
    Draft,
    Ongoing,
    Solved,
}

#[serde_as]
#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    #[serde_as(as = "DisplayFromStr")]
    quest_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    poster_id: i64,
    title: String,
    summary: String,
    details: String,
    status: QuestStatus,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

#[axum::debug_handler]
pub async fn get(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
) -> BackendResult<Json<Quest>> {
    let result = query_as!(
        Quest,
        "SELECT quest_id, poster_id, title, summary, details, status as \"status: _\", created_at \
        FROM quests \
        WHERE quest_id=$1",
        quest_id
    )
    .fetch_one(&state.db_pool)
    .await;
    match result {
        Ok(quest) => {
            // If Quest is public
            if quest.status != QuestStatus::Draft {
                return Ok(Json(quest));
            }
            // If Quest is owned by current user
            if let Ok(id) = helper::resolve_me_id(&session).await
                && id == quest.poster_id
            {
                return Ok(Json(quest));
            }
            Err(BackendError::NotFound("Quest".to_string()))
        }
        Err(sqlx::Error::RowNotFound) => Err(BackendError::NotFound("Quest".to_string())),
        Err(e) => Err(e.into()),
    }
}

#[derive(specta::Type, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestRequest {
    #[validate(custom(function = "validate_title"))]
    pub title: String,
}

#[axum::debug_handler]
pub async fn create(
    session: Session,
    State(state): State<AppState>,
    Valid(Json(req)): Valid<Json<CreateQuestRequest>>,
) -> BackendResult<Json<i64>> {
    let trimmed_title = req.title.trim();

    let id = helper::resolve_me_id(&session).await?;
    let quest_id: i64 = sqlx::query_scalar!(
        "INSERT INTO quests (poster_id, title) \
        VALUES ($1, $2) \
        RETURNING quest_id",
        id,
        trimmed_title,
    )
    .fetch_one(&state.db_pool)
    .await?;
    Ok(Json(quest_id))
}

#[derive(specta::Type, Validate, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestRequest {
    #[validate(custom(function = "validate_title"))]
    #[specta(optional)]
    pub title: Option<String>,
    #[specta(optional)]
    pub summary: Option<String>,
    #[specta(optional)]
    pub details: Option<String>,
    #[specta(optional)]
    pub status: Option<QuestStatus>,
}

#[axum::debug_handler]
pub async fn update(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
    Valid(Json(request)): Valid<Json<UpdateQuestRequest>>,
) -> BackendResult<StatusCode> {
    let poster_id = helper::resolve_me_id(&session).await?;
    let trim_title = request.title.map(|s| s.trim().to_string());
    sqlx::query!(
        "UPDATE quests \
        SET \
            title = COALESCE($1, title), \
            summary = COALESCE($2, summary), \
            details = COALESCE($3, details), \
            status = COALESCE($4, status) \
        WHERE quest_id = $5
        AND poster_id = $6",
        trim_title,
        request.summary,
        request.details,
        request.status as Option<QuestStatus>,
        quest_id,
        poster_id
    )
    .fetch_optional(&state.db_pool)
    .await?;
    Ok(StatusCode::NO_CONTENT)
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
