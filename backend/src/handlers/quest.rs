use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::{query_as, query_scalar};
use tower_sessions::Session;
use validator::Validate;

use crate::{
    AppState, BackendError, BackendResult,
    handlers::{
        common::{self},
        normvalid::{self, NormValid},
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

pub struct QuestRow {
    pub quest_id: i64,
    pub title: String,
    pub summary: String,
    pub details: String,
    pub techs: Vec<String>,
    pub status: QuestStatus,
    pub quest_created_at: time::OffsetDateTime,

    pub user_id: i64,
    pub github_id: i64,
    pub name: String,
    pub handle: String,
    pub user_created_at: time::OffsetDateTime,
}

impl From<QuestRow> for Quest {
    fn from(value: QuestRow) -> Self {
        Quest {
            quest_id: value.quest_id,
            poster: User {
                user_id: value.user_id,
                github_id: value.github_id,
                name: value.name,
                handle: value.handle,
                created_at: value.user_created_at,
            },
            title: value.title,
            summary: value.summary,
            details: value.details,
            techs: value.techs,
            status: value.status,
            created_at: value.quest_created_at,
        }
    }
}

#[axum::debug_handler]
pub async fn get(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
) -> BackendResult<Json<Quest>> {
    let result = query_as!(
        QuestRow,
        r#"
        SELECT q.quest_id, q.title, q.summary, q.details, q.techs,
        q.status as "status: QuestStatus", q.created_at as quest_created_at,
        u.user_id, u.github_id, u.name, u.handle, u.created_at as user_created_at
        FROM quests q
        JOIN users u ON u.user_id = q.poster_id
        WHERE q.quest_id=$1"#,
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

    let quest: Quest = result.into();

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

#[derive(Deserialize, Validate, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GetUserQuestParams {
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    #[validate(range(min = 1))]
    pub limit: Option<u32>,
}

#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct GetUserQuestResult {
    pub total: i64,
    pub is_last_page: bool,
    pub quests: Vec<Quest>,
}

#[axum::debug_handler]
pub async fn get_from_user(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
    NormValid(Json(params)): NormValid<Json<GetUserQuestParams>>,
) -> BackendResult<Json<GetUserQuestResult>> {
    let page = params.page.unwrap_or(1) as i64;
    let limit = params.limit.unwrap_or(20) as i64;
    let offset = (page - 1) * limit;
    let total = query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM quests
        WHERE poster_id=$1
            AND status != 'draft'"#,
        user_id,
    )
    .fetch_one(&state.db_pool)
    .await?
    .unwrap_or(0);
    let result = query_as!(
        QuestRow,
        r#"
        SELECT q.quest_id, q.title, q.summary, q.details, q.techs,
        q.status as "status: QuestStatus", q.created_at as quest_created_at,
        u.user_id, u.github_id, u.name, u.handle, u.created_at as user_created_at
        FROM quests q
        JOIN users u ON u.user_id = q.poster_id
        WHERE q.poster_id=$1
            AND q.status != 'draft'
        LIMIT $2 OFFSET $3"#,
        user_id,
        limit,
        offset
    )
    .fetch_all(&state.db_pool)
    .await?;

    let quests: Vec<Quest> = result.into_iter().map(|q| q.into()).collect();
    let is_last_page = offset + quests.len() as i64 >= total;
    return Ok(Json(GetUserQuestResult {
        total,
        is_last_page,
        quests,
    }));
}

#[derive(specta::Type, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

#[axum::debug_handler]
pub async fn create(
    session: Session,
    State(state): State<AppState>,
    NormValid(Json(req)): NormValid<Json<CreateQuestRequest>>,
) -> BackendResult<Json<i64>> {
    let trimmed_title = req.title.trim();

    let id = common::resolve_me_id(&session).await?;
    let quest_id: i64 = sqlx::query_scalar!(
        r#"
        INSERT INTO quests (poster_id, title)
        VALUES ($1, $2)
        RETURNING quest_id"#,
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
    #[validate(length(min = 1, max = 70))]
    #[specta(optional)]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 150))]
    #[specta(optional)]
    pub summary: Option<String>,
    #[validate(length(min = 1, max = 10000))]
    #[specta(optional)]
    pub details: Option<String>,
    #[specta(optional)]
    pub status: Option<QuestStatus>,
    #[validate(custom(function = "normvalid::techs"))]
    #[specta(optional)]
    pub techs: Option<Vec<String>>,
}

#[axum::debug_handler]
pub async fn update(
    session: Session,
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
    NormValid(Json(request)): NormValid<Json<UpdateQuestRequest>>,
) -> BackendResult<StatusCode> {
    let poster_id = common::resolve_me_id(&session).await?;
    sqlx::query!(
        r#"
        UPDATE quests
        SET
            title = COALESCE($1, title),
            summary = COALESCE($2, summary),
            details = COALESCE($3, details),
            status = COALESCE($4, status),
            techs = COALESCE($5, techs)
        WHERE quest_id = $6
        AND poster_id = $7"#,
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

#[derive(Deserialize, Validate, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverQuestParams {
    pub query: Option<String>,
    #[validate(custom(function = "normvalid::techs"))]
    pub techs: Option<Vec<String>>,
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    #[validate(range(min = 1))]
    pub limit: Option<u32>,
}

#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverQuestResult {
    pub total: i64,
    pub is_last_page: bool,
    pub quests: Vec<Quest>,
}

pub async fn discover(
    State(state): State<AppState>,
    NormValid(Json(params)): NormValid<Json<DiscoverQuestParams>>,
) -> BackendResult<Json<DiscoverQuestResult>> {
    let techs = params.techs.clone().unwrap_or_default();
    let page = params.page.unwrap_or(1) as i64;
    let limit = params.limit.unwrap_or(20) as i64;
    let offset = (page - 1) * limit;
    let query = params.query.unwrap_or_default();
    let query_pattern = format!("%{}%", query);
    let count = query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM quests
        WHERE ($1 = '' OR title <% $1 OR summary <% $1 OR title ILIKE $3 OR summary ILIKE $3)
            AND techs @> $2
            AND status != 'draft'"#,
        query,
        &techs,
        query_pattern
    )
    .fetch_one(&state.db_pool)
    .await?
    .unwrap_or(0);
    let result = query_as!(
        QuestRow,
        r#"
        SELECT q.quest_id, q.title, q.summary, q.details, q.techs,
        q.status as "status: QuestStatus", q.created_at as quest_created_at,
        u.user_id, u.github_id, u.name, u.handle, u.created_at as user_created_at
        FROM quests q
        JOIN users u ON u.user_id = q.poster_id
        WHERE ($1 = '' OR q.title <% $1 OR q.summary <% $1 OR q.title ILIKE $3 OR q.summary ILIKE $3)
            AND q.techs @> $2
            AND q.status != 'draft'
        ORDER BY q.quest_id
        LIMIT $4 OFFSET $5"#,
        query,
        &techs,
        query_pattern,
        limit,
        offset
    )
    .fetch_all(&state.db_pool)
    .await?;
    let quests: Vec<Quest> = result.into_iter().map(|q| q.into()).collect();
    let is_last_page = offset + quests.len() as i64 >= count;
    Ok(Json(DiscoverQuestResult {
        total: count,
        is_last_page,
        quests,
    }))
}
