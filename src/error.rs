use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::model::api::ErrorDto;

#[derive(Error, Debug)]
pub enum AppError {
    // 5xx Errors
    #[error("{0}")]
    Internal(String),
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid value for environment variable {var}: {reason}")]
    InvalidEnvVar {
        /// Name of the environment variable with invalid value.
        var: String,
        /// Explanation of why the value is invalid.
        reason: String,
    },
    #[error(transparent)]
    Dotenv(#[from] dotenvy::Error),
    #[error(transparent)]
    Database(#[from] tokio_postgres::Error),
    #[error(transparent)]
    DatabasePool(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    CreateDatabasePool(#[from] deadpool_postgres::CreatePoolError),
    #[error(transparent)]
    Redis(#[from] tower_sessions_redis_store::fred::prelude::Error),
    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            err => InternalServerError(err).into_response(),
        }
    }
}

/// Wrapper type for converting any displayable error into a 500 Internal Server Error response.
///
/// This struct logs the error message and returns a generic "Internal server error" message
/// to the client to avoid leaking implementation details. Used as a fallback for errors that
/// don't have specific HTTP response mappings.
struct InternalServerError<E>(pub E);

/// Converts wrapped errors into 500 Internal Server Error responses.
///
/// Logs the full error message for debugging, but returns a generic error message to the
/// client to avoid exposing internal implementation details or sensitive information.
///
/// # Arguments
/// - `E` - Any type that implements `Display` (typically an error type)
///
/// # Returns
/// A 500 Internal Server Error response with a generic error message JSON body
impl<E: std::fmt::Display> IntoResponse for InternalServerError<E> {
    fn into_response(self) -> Response {
        tracing::error!("Internal server error: {}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: "Internal server error".to_string(),
            }),
        )
            .into_response()
    }
}
