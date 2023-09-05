use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, QueryBuilder};
use tracing::{debug, trace};

use crate::{error::PersistenceError, Db};

use super::replay_report::ReplayReportDto;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDto {
    pub receiver: String,
    pub sender: String,
    pub body: String,
    pub tick: usize,
    pub player_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponseDto {
    pub body: String,
    pub tick: i32,
    pub relic_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub match_relic_id: i32,
    pub player_relic_id: i32,
    pub body: String,
    pub tick: i32,
}

impl Message {
    pub async fn read_many(match_relic_id: i32, db: &Db) -> Result<Vec<Message>, PersistenceError> {
        debug!("Fetching messages for game {}", match_relic_id);

        let sql = "SELECT * FROM message WHERE match_relic_id = $1";
        let messages = sqlx::query_as(sql)
            .bind(match_relic_id)
            .fetch_all(db)
            .await?;
        debug!("Done fetching messages for game: {}", match_relic_id);

        Ok(messages)
    }

    pub async fn create_many(game: &ReplayReportDto, db: &Db) -> Result<(), PersistenceError> {
        if game.messages.is_empty() {
            return Ok(());
        }

        let match_id = game.id.clone().parse::<i32>().unwrap();

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO message(match_relic_id, player_relic_id, body, tick) ");

        query_builder.push_values(&game.messages, |mut b, message| {
            trace!("Message to be pushed: {:?}", message);
            let player_relic_id = game
                .players
                .iter()
                .find(|player| player.name.eq(&message.sender))
                .map(|player| player.relic_id)
                .unwrap();

            b.push_bind(match_id)
                .push_bind(player_relic_id as i32)
                .push_bind(message.body.clone())
                .push_bind(message.tick as i32);
        });

        let query = query_builder.build();
        query.execute(db).await?;

        Ok(())
    }
}
