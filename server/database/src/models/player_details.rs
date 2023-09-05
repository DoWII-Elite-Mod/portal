use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDetailsDto {
    pub relic_id: i32,
    pub steam_id: i32,
    pub name: String,
    pub socials: PlayerSocials,
    pub rank: Option<i64>,
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
    pub main_faction: String,
    pub wins: i32,
    pub games: i32,
    pub top_opponents: Vec<TopOpponents>,
}

#[derive(Debug, Default, Deserialize, Serialize, FromRow)]
pub struct PlayerSocials {
    pub twitch_id: Option<String>,
    pub youtube_id: Option<String>,
    pub discord_id: Option<String>,
    pub forum_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TopOpponents {
    relic_id: i32,
    steam_id: i32,
    name: String,
    games: i64,
    wins: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlayerRating {
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
}

#[derive(Debug, FromRow)]
pub struct PlayerWinsAndMainFaction {
    pub winner: bool,
    pub race_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlayerRank {
    pub relic_id: i32,
    pub rank: Option<i64>,
    pub rating: f64,
    pub deviation: f64,
    pub volatility: f64,
}
