use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error as ThisError;

use crate::{
    error::PersistenceError, repository::player_repository, types::pagination::Pagination, Db,
};

use super::{
    player_details::{
        PlayerDetailsDto, PlayerRank, PlayerRating, PlayerSocials, PlayerWinsAndMainFaction,
    },
    replay_report::ReplayReporterPlayerDto,
};

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub relic_id: i32,
    pub steam_id: i32,
    pub name: String,
}

#[derive(Debug, ThisError)]
pub enum PlayerError {
    #[error("Player not found: {0}")]
    PlayerNotFound(i32),
    #[error("Rank could not be calculated")]
    RankNotCalculated,
    #[error("Unknown error with player entity: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for PlayerError {
    fn from(error: sqlx::Error) -> Self {
        Self::Unknown(error.to_string())
    }
}

impl Default for PlayerRating {
    fn default() -> Self {
        PlayerRating {
            rating: 1500.,
            deviation: 350.,
            volatility: 0.06,
        }
    }
}

impl From<ReplayReporterPlayerDto> for Player {
    fn from(player: ReplayReporterPlayerDto) -> Self {
        Self {
            relic_id: player.relic_id as i32,
            steam_id: player.steam_id as i32,
            name: player.name,
        }
    }
}

impl Player {
    pub async fn read(id: i32, db: &Db) -> Result<Self, PersistenceError> {
        let player = player_repository::get_player(id, db).await?;

        Ok(player)
    }

    pub async fn read_many(pagination: Pagination, db: &Db) -> Result<Vec<Self>, PersistenceError> {
        let limit = pagination.size;
        let offset = pagination.size * (pagination.page - 1);

        let players = player_repository::get_players(limit, offset, db).await?;

        Ok(players)
    }

    pub async fn create(player: Player, db: &Db) -> Result<(), PersistenceError> {
        player_repository::create_player(player, db).await?;

        Ok(())
    }

    pub async fn find_by_id(id: i32, db: &Db) -> Option<Player> {
        player_repository::find_by_id(id, db).await
    }

    pub async fn get_rating(id: i32, db: &Db) -> Result<PlayerRating, PersistenceError> {
        let rating = player_repository::get_rating(id, db).await?;

        match rating {
            Some(rating) => Ok(rating),
            None => Ok(PlayerRating::default()),
        }
    }

    pub async fn get_ratings(id: i32, db: &Db) -> Result<Vec<PlayerRating>, PersistenceError> {
        let ratings = player_repository::get_ratings(id, db).await?;

        Ok(ratings)
    }

    pub async fn get_details(id: i32, db: &Db) -> Result<PlayerDetailsDto, PersistenceError> {
        let player = player_repository::get_player(id, db).await?;
        let socials =
            sqlx::query_as::<_, PlayerSocials>("SELECT * FROM socials WHERE relic_id = $1")
                .bind(id)
                .fetch_optional(db)
                .await?
                .unwrap_or_default();
        let PlayerRating {
            rating,
            deviation,
            volatility,
        } = match player_repository::get_rating(id, db).await? {
            Some(rating) => rating,
            None => PlayerRating::default(),
        };
        let wins_and_factions = sqlx::query_as::<_, PlayerWinsAndMainFaction>("SELECT winner, race_name FROM player LEFT JOIN player_match ON player.relic_id = player_match.player_id LEFT JOIN hero on player_match.hero_id = hero.id WHERE relic_id = $1").bind(id).fetch_all(db).await?;

        let games = wins_and_factions
            .iter()
            .collect::<Vec<&PlayerWinsAndMainFaction>>()
            .len() as i32;

        let wins = wins_and_factions
            .iter()
            .filter(|game| game.winner)
            .collect::<Vec<&PlayerWinsAndMainFaction>>()
            .len() as i32;

        let main_faction = Player::get_main_faction(&wins_and_factions);

        let rank = Player::get_rank(id, db).await?;

        let top_opponents = player_repository::get_top_opponents(id, db).await?;

        let details = PlayerDetailsDto {
            relic_id: player.relic_id,
            steam_id: player.steam_id,
            name: player.name,
            socials,
            rank,
            rating,
            deviation,
            volatility,
            main_faction,
            wins,
            games,
            top_opponents,
        };

        Ok(details)
    }

    async fn get_rank(id: i32, db: &Db) -> Result<Option<i64>, PlayerError> {
        let ranks = sqlx::query_as::<_, PlayerRank>(
            "WITH ratings AS (
            SELECT DISTINCT ON (relic_id) relic_id, rating, deviation, volatility, timestamp
            FROM rating
            ORDER BY relic_id, timestamp DESC
            ) SELECT
            relic_id, rating,
            deviation,
            volatility,
            ROW_NUMBER () OVER (ORDER BY rating DESC) as rank
            FROM ratings;",
        )
        .fetch_all(db)
        .await?;

        let rank = ranks
            .iter()
            .find(|ranks| ranks.relic_id == id)
            .map(|player| player.rank);

        match rank {
            Some(rank) => Ok(rank),
            None => Ok(None),
        }
    }

    fn get_main_faction(games: &[PlayerWinsAndMainFaction]) -> String {
        let mut hash_map = HashMap::new();

        for faction in games.iter().map(|game| &game.race_name) {
            *hash_map.entry(faction).or_insert(0) += 1;
        }

        hash_map
            .into_iter()
            .max_by(|&a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| String::from(k))
            .unwrap()
    }
}
