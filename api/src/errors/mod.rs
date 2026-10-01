use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

#[derive(Debug)]
pub enum ApiError {
    Db(sqlx::Error),
    NotFound,
    BadRequest(String),
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Db(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
        };

        let body = Json(ErrorBody { error: msg });
        (status, body).into_response()
    }
}
