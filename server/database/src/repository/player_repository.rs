use crate::{
    error::PersistenceError,
    models::{
        player::{Player, PlayerError},
        player_details::{PlayerRating, TopOpponents},
    },
    Db,
};

pub async fn get_player(id: i32, db: &Db) -> Result<Player, PersistenceError> {
    let player = sqlx::query_as("SELECT * FROM player WHERE relic_id = $1")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(player)
}

pub async fn get_players(
    limit: i64,
    offset: i64,
    db: &Db,
) -> Result<Vec<Player>, PersistenceError> {
    let players = sqlx::query_as::<_, Player>("SELECT * FROM player LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await?;

    Ok(players)
}

pub async fn create_player(player: Player, db: &Db) -> Result<(), PersistenceError> {
    sqlx::query("INSERT INTO player (relic_id, steam_id, name) VALUES($1, $2, $3)")
        .bind(player.relic_id)
        .bind(player.steam_id)
        .bind(player.name)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn find_by_id(id: i32, db: &Db) -> Option<Player> {
    get_player(id, db).await.ok()
}

pub async fn get_rating(id: i32, db: &Db) -> Result<Option<PlayerRating>, PersistenceError> {
    let rating = sqlx::query_as::<_, PlayerRating>(
      "SELECT rating, deviation, volatility FROM rating WHERE relic_id = $1 ORDER BY timestamp DESC",
  )
  .bind(id)
  .fetch_optional(db)
  .await?;

    Ok(rating)
}

pub async fn get_ratings(id: i32, db: &Db) -> Result<Vec<PlayerRating>, PersistenceError> {
    let ratings = sqlx::query_as::<_, PlayerRating>(
        "SELECT rating, deviation, volatility FROM rating WHERE relic_id = $1 ORDER BY timestamp DESC",
    )
    .bind(id)
    .fetch_all(db)
    .await;

    match ratings {
        Ok(ratings) => Ok(ratings),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(Vec::new()),
            _ => Err(PersistenceError::Database(e.to_string())),
        },
    }
}

pub async fn get_top_opponents(id: i32, db: &Db) -> Result<Vec<TopOpponents>, PlayerError> {
    let sql = "SELECT * FROM (
SELECT b.player_id as relic_id, COUNT(*) as games, SUM(a.winner::int) as wins
FROM player_match as a
JOIN player_match as b ON a.match_id = b.match_id AND a.player_id <> b.player_id
WHERE a.player_id = $1
GROUP BY relic_id ) as x
JOIN player ON x.relic_id = player.relic_id
ORDER BY games DESC
LIMIT 3";

    let top_opponents = sqlx::query_as::<_, TopOpponents>(sql)
        .bind(id)
        .fetch_all(db)
        .await?;

    Ok(top_opponents)
}
