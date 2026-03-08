use std::sync::Arc;

use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};

use crate::{
    db,
    error::DbError,
    models::{CreateUserRequest, User},
};

pub async fn get_user_count(State(pool): State<Arc<Pool<Postgres>>>) -> Result<String, DbError> {
    db::get_user_count(&pool).await.map(|n| n.to_string())
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<User> {
    let new_user = User {
        id: 1,
        github_id: payload.github_id,
        username: payload.username,
    };
    Json(new_user)
}
