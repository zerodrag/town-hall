use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::{PgPool, query_as, query_scalar};
use tower_sessions::Session;
use validator::Validate;

use crate::{
    AppState, BackendError, BackendResult,
    handlers::{
        common,
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
        u.user_id, u.github_id, u.handle, u.created_at as user_created_at
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
pub struct SearchQuestParams {
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
pub struct SearchQuestResult {
    pub total: i64,
    pub quests: Vec<Quest>,
}

pub async fn discover(
    State(state): State<AppState>,
    NormValid(Query(params)): NormValid<Query<SearchQuestParams>>,
) -> BackendResult<Json<SearchQuestResult>> {
    let techs = params.techs.clone().unwrap_or_default();
    let page = params.page.unwrap_or(1) as i64;
    let limit = params.limit.unwrap_or(20) as i64;
    let offset = (page - 1) * limit;
    if let Some(query) = params.query {
        discover_search(query, techs, offset, limit, &state.db_pool).await
    } else {
        discover_random(techs, offset, limit, &state.db_pool).await
    }
}

pub async fn discover_search(
    query: String,
    techs: Vec<String>,
    offset: i64,
    limit: i64,
    pool: &PgPool,
) -> BackendResult<Json<SearchQuestResult>> {
    let query_pattern = format!("%{}%", query);
    let count = query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM quests
        WHERE (title <% $1 OR summary <% $1 OR title ILIKE $3 OR summary ILIKE $3)
            AND techs @> $2
            AND status != 'draft'"#,
        query,
        &techs,
        query_pattern
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    let result = query_as!(
        QuestRow,
        r#"
        SELECT q.quest_id, q.title, q.summary, q.details, q.techs,
        q.status as "status: QuestStatus", q.created_at as quest_created_at,
        u.user_id, u.github_id, u.handle, u.created_at as user_created_at
        FROM quests q
        JOIN users u ON u.user_id = q.poster_id
        WHERE (q.title <% $1 OR q.summary <% $1 OR q.title ILIKE $3 OR q.summary ILIKE $3)
            AND q.techs @> $2
            AND q.status != 'draft'
        ORDER BY GREATEST(word_similarity(q.title, $1), word_similarity(q.summary, $1)) DESC
        LIMIT $4 OFFSET $5"#,
        query,
        &techs,
        query_pattern,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let quests: Vec<Quest> = result.into_iter().map(Quest::from).collect();
    Ok(Json(SearchQuestResult { total: count, quests }))
}

pub async fn discover_random(
    techs: Vec<String>,
    offset: i64,
    limit: i64,
    pool: &PgPool,
) -> BackendResult<Json<SearchQuestResult>> {
    let count = query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM quests
        WHERE techs @> $1
            AND status != 'draft'"#,
        &techs
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);
    let result = query_as!(
        QuestRow,
        r#"
        SELECT q.quest_id, q.title, q.summary, q.details, q.techs,
        q.status as "status: QuestStatus", q.created_at as quest_created_at,
        u.user_id, u.github_id, u.handle, u.created_at as user_created_at
        FROM quests q
        JOIN users u ON u.user_id = q.poster_id
        WHERE q.techs @> $1
            AND status != 'draft'
        ORDER BY extract(epoch from q.created_at) * random() DESC
        LIMIT $2 OFFSET $3"#,
        &techs,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let quests: Vec<Quest> = result.into_iter().map(Quest::from).collect();
    Ok(Json(SearchQuestResult { total: count, quests }))
}
