use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};
use sqlx::FromRow;

use crate::{
    models::{action::ActionDto, message::MessageResponseDto, replay_report::ReplayReportDto},
    types::pagination::Pagination,
};

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Game {
    pub relic_id: i32,
    pub map_id: i32,
    pub ticks: i32,
    pub league_id: Option<i32>,
    pub reported_at: DateTime<Utc>,
    pub mod_version: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct GameListItemDto {
    pub match_id: i32,
    pub player_id: i32,
    pub winner: bool,
    pub hero_id: i32,
    pub opponent_id: i32,
    pub opponent_hero_id: i32,
    pub relic_id: i32,
    pub map_id: i32,
    pub ticks: i32,
    pub league_id: Option<i32>,
    pub reported_at: DateTime<Utc>,
    pub mod_version: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullGameDto {
    pub map: String,
    pub relic_id: i32,
    pub ticks: i32,
    pub league_id: Option<i32>,
    pub reported_at: DateTime<Utc>,
    pub mod_version: i32,
    pub players: Vec<FullGamePlayer>,
    pub actions: Vec<ActionDto>,
    pub messages: Vec<MessageResponseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullGamePlayer {
    pub relic_id: i32,
    pub name: String,
    pub hero: String,
}

// region: GamesQueryParameters

#[derive(Debug, Deserialize)]
pub struct GamesQueryParameters {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub player_id: Option<i32>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

// endregion: GamesQueryParameters

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FullGameFromDB {
    pub relic_id: i32,
    pub map: String,
    pub ticks: i32,
    pub league_id: Option<i32>,
    pub reported_at: DateTime<Utc>,
    pub mod_version: i32,
    pub player_relic_id: i32,
    pub player_name: String,
    pub hero_name: String,
}

impl From<Vec<FullGameFromDB>> for FullGameDto {
    fn from(value: Vec<FullGameFromDB>) -> Self {
        FullGameDto {
            map: value.get(0).unwrap().map.clone(),
            relic_id: value.get(0).unwrap().relic_id,
            ticks: value.get(0).unwrap().ticks,
            league_id: value.get(0).unwrap().league_id,
            reported_at: value.get(0).unwrap().reported_at,
            mod_version: value.get(0).unwrap().mod_version,
            players: value
                .iter()
                .map(|row| FullGamePlayer {
                    hero: row.hero_name.clone(),
                    name: row.player_name.clone(),
                    relic_id: row.player_relic_id,
                })
                .collect(),
            actions: vec![],
            messages: vec![],
        }
    }
}

impl From<ReplayReportDto> for Game {
    fn from(game: ReplayReportDto) -> Self {
        Self {
            relic_id: game.id.parse::<i32>().unwrap(),
            map_id: 1,
            ticks: game.ticks as i32,
            league_id: None,
            reported_at: DateTime::default(),
            mod_version: game.mod_version as i32,
        }
    }
}
