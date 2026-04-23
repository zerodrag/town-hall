use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::query;
use tower_sessions::Session;
use validator::{Validate, ValidationError};

use crate::{
    AppState, BackendError, BackendResult,
    handlers::{
        common::{self, NormValidJson, Normalize},
        user::User,
    },
};

#[derive(PartialEq, Debug, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[sqlx(type_name = "quest_status", rename_all = "snake_case")]
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
    pub quest_id: i64,
    pub poster: User,
    pub title: String,
    pub summary: String,
    pub details: String,
    pub techs: Vec<String>,
    pub status: QuestStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
}

#[axum::debug_handler]
pub async fn get(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
) -> BackendResult<Json<Quest>> {
    let result = query!(
        "SELECT q.quest_id, q.title, q.summary, q.details, q.techs, \
        q.status as \"status: QuestStatus\", q.created_at as quest_created_at, \
        u.user_id, u.github_id, u.handle, u.created_at as user_created_at \
        FROM quests q \
        JOIN users u ON u.user_id = q.poster_id
        WHERE q.quest_id=$1",
        quest_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            BackendError::NotFound("Quest".to_string())
        } else {
            e.into()
        }
    })?;

    let quest = Quest {
        quest_id: result.quest_id,
        poster: User {
            user_id: result.user_id,
            github_id: result.github_id,
            handle: result.handle,
            created_at: result.user_created_at,
        },
        title: result.title,
        summary: result.summary,
        details: result.details,
        techs: result.techs,
        status: result.status,
        created_at: result.quest_created_at,
    };

    // If Quest is public
    if quest.status != QuestStatus::Draft {
        return Ok(Json(quest));
    }
    // If Quest is owned by current user
    if let Ok(id) = common::resolve_me_id(&session).await
        && id == quest.poster.user_id
    {
        return Ok(Json(quest));
    }
    // If Quest is neither public nor owned by user
    Err(BackendError::NotFound("Quest".to_string()))
}

#[derive(specta::Type, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

impl Normalize for CreateQuestRequest {
    fn normalize(&mut self) {
        self.title = self.title.trim().to_string();
    }
}

#[axum::debug_handler]
pub async fn create(
    session: Session,
    State(state): State<AppState>,
    NormValidJson(req): NormValidJson<CreateQuestRequest>,
) -> BackendResult<Json<i64>> {
    let trimmed_title = req.title.trim();

    let id = common::resolve_me_id(&session).await?;
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
    #[validate(length(min = 1, max = 100))]
    #[specta(optional)]
    pub title: Option<String>,
    #[validate(length(max = 300))]
    #[specta(optional)]
    pub summary: Option<String>,
    #[validate(length(max = 10000))]
    #[specta(optional)]
    pub details: Option<String>,
    #[specta(optional)]
    pub status: Option<QuestStatus>,
    #[validate(custom(function = "validate_techs"))]
    #[specta(optional)]
    pub techs: Option<Vec<String>>,
}

fn validate_techs(techs: &Vec<String>) -> Result<(), ValidationError> {
    if techs.len() > 10 {
        let err = ValidationError::new("invalid_tech_count");
        return Err(err.with_message("There must be no more than 10 Techs".into()));
    }
    if !techs.iter().all_unique() {
        let err = ValidationError::new("techs_not_all_unique");
        return Err(err.with_message("Techs must be all unique".into()));
    }
    for tech in techs {
        if tech.len() > 15 {
            let err = ValidationError::new("tech_invalid_length");
            return Err(err.with_message("Tech must be no longer than 15 characters".into()));
        }
        if !tech.chars().all(|c| c.is_alphanumeric() || c == '-') {
            let err = ValidationError::new("tech_invalid_character");
            return Err(err.with_message("Tech must not contain non-alphanumeric and non-hyphen (-) characters".into()));
        }
    }
    Ok(())
}

impl Normalize for UpdateQuestRequest {
    fn normalize(&mut self) {
        self.title.as_mut().map(|title| common::trim_whitespace(title));
        self.techs
            .as_mut()
            .map(|techs| techs.iter().map(|tech| common::trim_whitespace(tech)));
    }
}

#[axum::debug_handler]
pub async fn update(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
    NormValidJson(request): NormValidJson<UpdateQuestRequest>,
) -> BackendResult<StatusCode> {
    let poster_id = common::resolve_me_id(&session).await?;
    sqlx::query!(
        "UPDATE quests \
        SET \
            title = COALESCE($1, title), \
            summary = COALESCE($2, summary), \
            details = COALESCE($3, details), \
            status = COALESCE($4, status), \
            techs = COALESCE($5, techs) \
        WHERE quest_id = $6
        AND poster_id = $7",
        request.title,
        request.summary,
        request.details,
        request.status as Option<QuestStatus>,
        request.techs.as_deref(),
        quest_id,
        poster_id
    )
    .fetch_optional(&state.db_pool)
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
