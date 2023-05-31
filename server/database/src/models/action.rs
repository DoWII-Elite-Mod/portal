use serde::{Deserialize, Serialize};
use sqlx::{Execute, FromRow, Postgres, QueryBuilder};
use thiserror::Error as ThisError;
use tracing::{debug, trace, warn};

use crate::Db;

#[derive(Debug, ThisError)]
pub enum ActionError {
    #[error("Action data not saved")]
    ActionsNotSaved,
    #[error("Action data not found")]
    ActionsNotFound,
    #[error("Unkown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for ActionError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::ActionsNotFound,
            _ => Self::Unknown(error.to_string()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Action {
    pub relic_id: i32,
    pub match_relic_id: i32,
    pub tick: i32,
    pub data: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionDto {
    pub relic_id: i32,
    pub tick: i32,
    pub name: String,
    pub data: Vec<i32>,
}

impl Action {
    pub async fn read_many(match_relic_id: i32, db: &Db) -> Result<Vec<Action>, ActionError> {
        debug!("Fetching actions for game: {}", match_relic_id);

        let sql = "SELECT * FROM action WHERE match_relic_id = $1";
        let actions = sqlx::query_as(sql)
            .bind(match_relic_id)
            .fetch_all(db)
            .await?;
        debug!("Done fetching actions for game: {}", match_relic_id);

        Ok(actions)
    }

    pub async fn create_many(
        actions: &Vec<ActionDto>,
        match_relic_id: i32,
        db: &Db,
    ) -> Result<(), ActionError> {
        if actions.is_empty() {
            warn!("No actions were recorded for game {}", match_relic_id);
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO action(relic_id, match_relic_id, tick, data) ");

        query_builder.push_values(actions, |mut b, action| {
            trace!("Action to be pushed: {:?}", action);
            b.push_bind(action.relic_id)
                .push_bind(match_relic_id)
                .push_bind(action.tick)
                .push_bind::<Vec<i32>>(action.data.clone());
        });

        let query = query_builder.build();
        debug!("{}", query.sql());
        query.execute(db).await?;

        Ok(())
    }
}
