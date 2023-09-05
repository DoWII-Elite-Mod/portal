use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{map::Map, race::Race};

#[derive(Debug, Serialize)]
pub struct RecentGameDto {
    pub map: Map,
    pub ticks: i32,
    pub winner: bool,
    pub players: Vec<RecentGamePlayers>,
    pub date_time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RecentGamePlayers {
    pub faction: Race,
    pub name: String,
    pub relic_id: i32,
}
