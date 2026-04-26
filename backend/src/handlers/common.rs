use tower_sessions::Session;

use crate::{BackendError, BackendResult};

/// Returns Ok(id) if logged in and Err((StatusCode::UNAUTHORIZED)) if not
pub async fn resolve_me_id(session: &Session) -> BackendResult<i64> {
    let id_result = session.get("user_id").await?;
    match id_result {
        Some(val) => Ok(val),
        None => Err(BackendError::Unauthorized),
    }
}

pub fn trim_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}
