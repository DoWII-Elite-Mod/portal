use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{error::PersistenceError, Db};

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct PlayerGame {
    pub player_id: i32,
    pub match_id: i32,
    pub winner: bool,
    pub hero_id: i32,
    pub team: i32,
}

impl PlayerGame {
    pub async fn create(player_game: PlayerGame, db: &Db) -> Result<(), PersistenceError> {
        let sql = "INSERT INTO player_match (player_id, match_id, winner, hero_id, team) VALUES($1, $2, $3, $4, $5)";
        sqlx::query(sql)
            .bind(player_game.player_id)
            .bind(player_game.match_id)
            .bind(player_game.winner)
            .bind(player_game.hero_id + 1)
            .bind(player_game.team)
            .fetch_optional(db)
            .await?;

        Ok(())
    }

    pub async fn read_many(db: &Db) -> Result<Vec<PlayerGame>, PersistenceError> {
        let sql = "SELECT * FROM player_match";

        let res = sqlx::query_as::<_, PlayerGame>(sql).fetch_all(db).await?;

        Ok(res)
    }
}
