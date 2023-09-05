use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error as ThisError;
use tracing::debug;

use crate::{error::PersistenceError, Db};

use super::{
    action::{Action, ActionDto},
    games::types::Game,
    map::Map,
    message::{Message, MessageDto},
    player::Player,
    player_game::PlayerGame,
};

// region: Models
#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct ReplayReportDto {
    pub aborted: bool,
    pub actions: Vec<ActionDto>,
    pub dev: bool,
    pub id: String,
    pub map: String,
    pub reporter: ReplayReportReporterDto,
    pub replay: String,
    pub mod_version: usize,
    pub ranked: bool,
    pub league: bool,
    pub frames: usize,
    pub ticks: usize,
    pub players: Vec<ReplayReporterPlayerDto>,
    pub messages: Vec<MessageDto>,
    //pub observers: Vec<>
    pub winner: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayReportReporterDto {
    pub date: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplayReporterPlayerDto {
    pub relic_id: usize,
    pub hero: usize,
    pub race: usize,
    pub name: String,
    pub steam_id: usize,
    pub team: usize,
    pub sim_id: usize,
    pub slot: usize,
}
// endregion: Models

#[derive(ThisError, Debug)]
pub enum ValidationError {
    #[error("Game is not ranked")]
    IsNotRanked,
    #[error("Game length is not correct. Ticks: {0} - Frames: {1}")]
    GameLengthNotCorrect(usize, usize),
    #[error("Game is not 1v1")]
    IsNot1v1,
    #[error("Game was aborted")]
    IsAborted,
    #[error("Game already exists")]
    Duplicate,
    #[error("Unknown validation error {0}")]
    Unknown(String),
}

type ValidationResult = Result<(), ValidationError>;

// region: Validation
impl ReplayReportDto {
    pub async fn validate(&self, db: &Db) -> ValidationResult {
        self.does_exist(db).await?;
        self.is_aborted()?;
        self.is_ranked()?;
        self.is_1v1()?;
        self.is_game_length_correct()?;

        Ok(())
    }

    async fn does_exist(&self, db: &Db) -> ValidationResult {
        match Game::find_by_id(self.id.parse::<i32>().unwrap(), db).await {
            Ok(res) => match res {
                Some(_) => Err(ValidationError::Duplicate),
                None => Ok(()),
            },
            Err(e) => Err(ValidationError::Unknown(e.to_string())),
        }
    }

    fn is_ranked(&self) -> ValidationResult {
        match self.ranked {
            true => Ok(()),
            false => Err(ValidationError::IsNotRanked),
        }
    }

    fn is_game_length_correct(&self) -> ValidationResult {
        match self.ticks.abs_diff(self.frames) < 10 {
            true => Ok(()),
            false => Err(ValidationError::GameLengthNotCorrect(
                self.ticks,
                self.frames,
            )),
        }
    }

    fn is_1v1(&self) -> ValidationResult {
        match self.players.len() == 2 {
            true => Ok(()),
            false => Err(ValidationError::IsNot1v1),
        }
    }

    fn is_aborted(&self) -> ValidationResult {
        match self.aborted {
            true => Err(ValidationError::IsAborted),
            false => Ok(()),
        }
    }
}
// endregion: Validation

// region: Implementation
impl ReplayReportDto {
    pub async fn create(db: &Db, game: Self) -> Result<(), PersistenceError> {
        let match_id = game.id.parse::<i32>().unwrap();

        game.validate(db).await?;

        // Find map with data.map
        let map = Map::find_by_name(&game.map, db).await?;

        // If league, find league id

        // Find players in DB and add them if they dont exist
        for player in &game.players {
            if Player::find_by_id(player.relic_id as i32, db)
                .await
                .is_none()
            {
                Player::create(player.clone().into(), db).await?
            };
        }

        Game::create(
            Game {
                relic_id: match_id,
                map_id: map.id,
                ticks: game.ticks as i32,
                league_id: None,
                reported_at: chrono::offset::Utc::now(),
                mod_version: game.mod_version as i32,
            },
            db,
        )
        .await?;

        for player in &game.players {
            let player_game = PlayerGame {
                player_id: player.relic_id as i32,
                match_id,
                winner: game.winner == player.team,
                hero_id: player.hero as i32,
                team: player.team as i32,
            };

            debug!("Writing PlayerGame: {:?}", player_game);
            PlayerGame::create(player_game, db).await?;
        }

        Action::create_many(&game.actions, match_id, db).await?;
        Message::create_many(&game, db).await?;

        Ok(())
    }
}

// endregion: Implementation
