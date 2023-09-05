use thiserror::Error as ThisError;

use crate::models::{
    action::ActionError, map::MapError, player::PlayerError, replay_report::ValidationError,
};

#[derive(ThisError, Debug)]
pub enum PersistenceError {
    #[error("Generic Persistence Error: {0}")]
    Generic(String),
    #[error("Database Error: {0}")]
    Database(String),
    #[error("Migrations Error: {0}")]
    Migrations(String),
    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationError),
    #[error("Map Error: {0}")]
    Map(#[from] MapError),
    #[error("Action Error: {0}")]
    Action(#[from] ActionError),
    #[error("Player Error: {0}")]
    Player(#[from] PlayerError),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e.to_string())
    }
}
