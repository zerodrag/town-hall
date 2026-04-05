use http::StatusCode;
use tower_sessions::Session;

/// Simple wrapper type. Requires T to implement IntoResponse to be used as handler response type.
pub type SimpResp<T> = Result<T, (StatusCode, &'static str)>;

pub async fn resolve_current_user_id(session: &Session) -> SimpResp<i64> {
    let id_result=resolve_session_key(session, "user_id").await;
    if let Err((StatusCode::NOT_FOUND, _)) = id_result {
        return Err((StatusCode::UNAUTHORIZED, "Not logged in"));
    }
    let Ok(id): Result<i64, _> = id_result?.parse() else {
        tracing::error!("Session user_id is not a number");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
    };
    Ok(id)
}

pub async fn resolve_session_key(session: &Session, key: &str) -> SimpResp<String>{
    let maybe_result: Option<String> = match session.get(key).await  {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("Session error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
        }
    };

    match maybe_result {
        Some(val) => Ok(val),
        None => {
            tracing::error!("Session key ({key}) does not exist");
            return Err((StatusCode::NOT_FOUND, "Session Key not found"));
        }
    }
}