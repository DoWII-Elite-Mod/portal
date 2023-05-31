use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error as ThisError;

use crate::{error::PersistenceError, Db};

#[derive(Debug, ThisError)]
pub enum MapError {
    #[error("Map not found")]
    MapNotFound,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for MapError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::MapNotFound,
            _ => Self::Unknown(error.to_string()),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Map {
    pub id: i32,
    pub file_name: String,
    pub screen_name: String,
    pub player_count: i32,
}

impl Map {
    pub async fn find_by_name(name: impl Into<String>, db: &Db) -> Result<Self, MapError> {
        let sql = "SELECT * FROM map WHERE file_name = $1";
        let map = sqlx::query_as(sql).bind(name.into()).fetch_one(db).await?;

        Ok(map)
    }

    pub async fn find_by_id(id: i32, db: &Db) -> Result<Self, MapError> {
        let sql = "SELECT * FROM map WHERE id = $1";
        let map = sqlx::query_as(sql).bind(id).fetch_one(db).await?;

        Ok(map)
    }

    pub async fn ready_many(db: &Db) -> Result<Vec<Map>, PersistenceError> {
        let sql = "SELECT * FROM map";
        let maps = sqlx::query_as::<_, Map>(sql).fetch_all(db).await?;

        Ok(maps)
    }
}
