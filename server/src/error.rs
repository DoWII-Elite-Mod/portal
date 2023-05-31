use std::env::VarError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use database::error::PersistenceError;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Generic Error: {0}")]
    Generic(String),
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
}

#[derive(Deserialize, Serialize)]
struct ErrorJson {
    error: String,
}

impl ErrorJson {
    fn from(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Self::Generic(e.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Generic(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson::from(message)),
            )
                .into_response(),
            Error::Persistence(error) => match error {
                PersistenceError::Generic(error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorJson::from(error)),
                )
                    .into_response(),
                PersistenceError::Database(error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorJson::from(error)),
                )
                    .into_response(),
                PersistenceError::Validation(error) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorJson::from(error.to_string())),
                )
                    .into_response(),
                PersistenceError::Map(error) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorJson::from(error.to_string())),
                )
                    .into_response(),
                PersistenceError::Action(error) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorJson::from(error.to_string())),
                )
                    .into_response(),
                PersistenceError::Player(error) => (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorJson::from(error.to_string())),
                )
                    .into_response(),
                PersistenceError::Migrations(error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorJson::from(error.to_string())),
                )
                    .into_response(),
            },
        }
    }
}
