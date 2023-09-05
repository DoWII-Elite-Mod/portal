use std::collections::HashMap;

use database::{
    error::PersistenceError,
    models::{player::Player, player_game::PlayerGame},
    Db,
};

use itertools::Itertools;
use skillratings::{
    glicko2::{glicko2_rating_period, Glicko2Config, Glicko2Rating},
    Outcomes,
};
use tracing::{debug, trace};

pub struct Ratings;

struct Matchup {
    player_id: i32,
    winner: bool,
    opponent_id: i32,
}

#[derive(Debug, Default)]
struct PlayerResult {
    pub results: Vec<(Glicko2Rating, Outcomes)>,
}

impl Ratings {
    pub async fn calculate(db: &Db) -> Result<(), PersistenceError> {
        let mut matches = PlayerGame::read_many(db).await?;
        let mut results: HashMap<i32, PlayerResult> = HashMap::new();

        matches.sort_by(|a, b| a.player_id.cmp(&b.player_id));

        // get unique player ids
        let unique_player_ids = matches
            .iter()
            .map(|p| p.player_id)
            .unique()
            .collect::<Vec<_>>();

        let matchups = matches
            .iter()
            .cartesian_product(&matches)
            .filter(|(a, b)| a.player_id != b.player_id)
            .filter(|(a, b)| a.match_id == b.match_id)
            .map(|(a, b)| Matchup {
                player_id: a.player_id,
                winner: a.winner,
                opponent_id: b.player_id,
            })
            .collect::<Vec<_>>();

        Ratings::collect_matchup_results(matchups, db, &mut results).await?;

        debug!("Starting to update ratings");

        Ratings::update_ratings(unique_player_ids, &results, db).await?;

        debug!("Done updating ratings");

        Ok(())
    }

    async fn update_ratings(
        unique_player_ids: Vec<i32>,
        results: &HashMap<i32, PlayerResult>,
        db: &Db,
    ) -> Result<(), PersistenceError> {
        for id in unique_player_ids {
            let current_rating = Player::get_rating(id, db).await?; // TODO: get list of ratings all at once beforehand
            let new_player_rating = glicko2_rating_period(
                &Glicko2Rating {
                    rating: current_rating.rating,
                    deviation: current_rating.deviation,
                    volatility: current_rating.volatility,
                },
                &results.get(&id).unwrap().results,
                &Glicko2Config::new(),
            );

            sqlx::query("INSERT INTO rating (relic_id, rating, deviation, volatility) VALUES ($1, $2, $3, $4)")
                .bind(id)
                .bind(new_player_rating.rating)
                .bind(new_player_rating.deviation)
                .bind(new_player_rating.volatility)
                .execute(db)
                .await?;

            trace!("New player ({:?}) rating: {:#?}", id, new_player_rating);
        }

        Ok(())
    }

    async fn collect_matchup_results(
        matchups: Vec<Matchup>,
        db: &Db,
        results: &mut HashMap<i32, PlayerResult>,
    ) -> Result<(), PersistenceError> {
        for matchup in &matchups {
            let Matchup {
                player_id,
                opponent_id,
                ..
            } = *matchup;

            let opponent_rating = Player::get_rating(opponent_id, db).await?; // TODO: get list of ratings all at once beforehand

            let outcome = match matchup.winner {
                true => Outcomes::WIN,
                false => Outcomes::LOSS,
            };

            match results.get_mut(&player_id) {
                Some(result) => result.results.push((
                    Glicko2Rating {
                        rating: opponent_rating.rating,
                        deviation: opponent_rating.deviation,
                        volatility: opponent_rating.volatility,
                    },
                    outcome,
                )),
                None => {
                    results.insert(player_id, PlayerResult::default());
                }
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn calculate_ratings() {
        let db = database::init_db().await.unwrap();
        Ratings::calculate(&db).await.unwrap();
    }
}
