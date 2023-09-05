use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{action::ActionDto, message::MessageResponseDto};

#[derive(Debug, Serialize)]
pub struct GameDetailsDto {
    pub map: String,
    pub relic_id: i32,
    pub ticks: i32,
    pub league_id: Option<i32>,
    pub reported_at: DateTime<Utc>,
    pub mod_version: i32,
    pub players: Vec<GameDetailsPlayerDto>,
    pub actions: Vec<ActionDto>,
    pub messages: Vec<MessageResponseDto>,
}

#[derive(Debug, Serialize)]
pub struct GameDetailsPlayerDto {
    pub relic_id: i32,
    pub name: String,
    pub hero_id: i32,
    pub hero_name: String,
}
