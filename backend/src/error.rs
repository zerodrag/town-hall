use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum DbError {
    UnexpectedValue(&'static str),
    Internal(sqlx::Error),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        match self {
            Self::UnexpectedValue(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            Self::Internal(ref e) => {
                // TODO: Log error
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal database error occurred",
                )
            }
            .into_response(),
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        Self::Internal(err)
    }
}
