use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Json};
use sea_orm::DbErr;
use serde_json::json;

pub enum AppError {
    DbError(DbErr),
    NotFound,
    InvalidCredentials,
    Validation(String),
    DuplicateEntry(String),
    InternalError(String),
    InvalidToken,
    Forbidden,
    AlreadyLogin,
    BadRequest(String),
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> AppError {
        AppError::DbError(err)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DbError(err) => write!(f, "Database error: {}", err),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::DuplicateEntry(msg) => write!(f, "Duplicate entry: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::InvalidToken => write!(f, "Invalid token"),
            AppError::AlreadyLogin => write!(f, "Already logged in"),
            AppError::Forbidden => write!(f, "Access forbidden"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DbError(err) =>
                match dotenvy::var("HOST_MODE").unwrap() == "development" {
                true => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Kesalahan database: {}", err)
                ),
                false => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Terjadi kesalahan pada server".to_string()
                )
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Data yang dicari tidak ditemukan".to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Kredensial tidak valid".to_string()),
            // Gunakan status 422 Unprocessable Entity untuk error validasi
            AppError::Validation(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            // Gunakan status 409 Conflict untuk data duplikat
            AppError::DuplicateEntry(message) => (StatusCode::CONFLICT, message),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Kredensial tidak valid".to_string()),
            AppError::InternalError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Gaboleh massuk".to_string()),
            AppError::AlreadyLogin => (StatusCode::FORBIDDEN, "Udah login".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}