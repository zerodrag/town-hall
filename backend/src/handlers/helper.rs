use http::StatusCode;
use tower_sessions::Session;

/// Simple wrapper type. Requires T to implement IntoResponse to be used as handler response type.
pub type SimpResp<T> = Result<T, (StatusCode, &'static str)>;

/// Returns Ok(id) if logged in and Err((StatusCode::UNAUTHORIZED)) if not
pub async fn resolve_current_user_id(session: &Session) -> SimpResp<i64> {
    let id_result = resolve_session_key::<i64>(session, "user_id").await?;
    match id_result {
        Some(val) => Ok(val),
        None => Err((StatusCode::UNAUTHORIZED, "Not logged in")),
    }
}

/// Returns Ok(Some(val)) if value found for key, Ok(None) if value not found, Err if session error happens
pub async fn resolve_session_key<T>(session: &Session, key: &str) -> SimpResp<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    let result: Option<T> = match session.get(key).await {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Session error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
        }
    };
    Ok(result)
}
