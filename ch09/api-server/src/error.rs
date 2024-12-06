use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum Error {
    Sqlx(StatusCode, String),
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Sqlx(StatusCode::INTERNAL_SERVER_ERROR, value.to_string()),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Sqlx(code, body) => (code, body).into_response(),
            Error::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
