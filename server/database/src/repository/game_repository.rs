use crate::{
    error::PersistenceError,
    models::games::types::{FullGameFromDB, Game, GameListItemDto},
    Db,
};

pub async fn get_games(
    limit: i64,
    offset: i64,
    db: &Db,
) -> Result<Vec<GameListItemDto>, PersistenceError> {
    let matches = sqlx::query_as::<_, GameListItemDto>("SELECT a.match_id, a.player_id, a.winner, a.hero_id, b.player_id as opponent_id, b.hero_id as opponent_hero_id, m.* FROM player_match as a
          JOIN player_match as b ON a.match_id = b.match_id AND a.player_id <> b.player_id
          JOIN match as m on a.match_id = m.relic_id
          ORDER BY m.reported_at DESC
          LIMIT $1
          OFFSET $2")
        .bind(limit)
        .bind(offset)
      .fetch_all(db)
      .await?;

    Ok(matches)
}

pub async fn get_games_for_player(
    limit: i64,
    offset: i64,
    id: i32,
    db: &Db,
) -> Result<Vec<GameListItemDto>, PersistenceError> {
    let matches = sqlx::query_as::<_, GameListItemDto>("SELECT a.match_id, a.player_id, a.winner, a.hero_id, b.player_id as opponent_id, b.hero_id as opponent_hero_id, m.* FROM player_match as a
        JOIN player_match as b ON a.match_id = b.match_id AND a.player_id <> b.player_id
        JOIN match as m on a.match_id = m.relic_id
        WHERE a.player_id = $1
        ORDER BY m.reported_at DESC
        LIMIT $2
        OFFSET $3")
    .bind(id)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await?;

    Ok(matches)
}

pub async fn get_game(id: i32, db: &Db) -> Result<Vec<FullGameFromDB>, PersistenceError> {
    let game = sqlx::query_as::<_, FullGameFromDB>("SELECT match.relic_id, screen_name as map, ticks, league_id, reported_at, mod_version, player.relic_id as player_relic_id, player.name as player_name, hero.name as hero_name FROM match
        LEFT JOIN player_match ON match.relic_id = player_match.match_id
        LEFT JOIN player ON player_match.player_id = player.relic_id
        LEFT JOIN map ON match.map_id = map.id
        LEFT JOIN hero on player_match.hero_id = hero.id
        WHERE player_match.match_id = $1")
    .bind(id)
    .fetch_all(db)
    .await?;

    Ok(game)
}

pub async fn find_game_by_id(id: i32, db: &Db) -> Result<Option<Game>, PersistenceError> {
    let game = sqlx::query_as::<_, Game>("SELECT * FROM match WHERE relic_id = $1")
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(game)
}

pub async fn create_game(game: Game, db: &Db) -> Result<(), PersistenceError> {
    let sql = "INSERT INTO match (relic_id, map_id, ticks, league_id, reported_at, mod_version) VALUES($1, $2, $3, $4, $5, $6)";
    sqlx::query(sql)
        .bind(game.relic_id)
        .bind(game.map_id)
        .bind(game.ticks)
        .bind(game.league_id)
        .bind(game.reported_at)
        .bind(game.mod_version)
        .fetch_optional(db)
        .await?;

    Ok(())
}
