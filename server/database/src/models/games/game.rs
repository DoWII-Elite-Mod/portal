use crate::{
    error::PersistenceError,
    models::{
        action::{Action, ActionDto},
        message::{Message, MessageResponseDto},
    },
    repository::game_repository,
    types::pageable::Pageable,
    Db,
};

use super::types::{FullGameDto, Game, GameListItemDto, GamesQueryParameters};

impl Game {
    pub async fn read_many(
        query_parameters: GamesQueryParameters,
        db: &Db,
    ) -> Result<Pageable<GameListItemDto>, PersistenceError> {
        let limit = query_parameters.pagination.size;
        let offset = query_parameters.pagination.size * (query_parameters.pagination.page - 1);

        match query_parameters.player_id {
            None => {
                let games = game_repository::get_games(limit, offset, db).await?;
                let number_of_elements = games.len();

                Ok(Pageable {
                    content: games,
                    pagination: query_parameters.pagination,
                    number_of_elements,
                })
            }
            Some(id) => {
                let games = game_repository::get_games_for_player(limit, offset, id, db).await?;
                let number_of_elements = games.len();

                Ok(Pageable {
                    content: games,
                    pagination: query_parameters.pagination,
                    number_of_elements,
                })
            }
        }
    }

    pub async fn read(id: i32, db: &Db) -> Result<FullGameDto, PersistenceError> {
        let rows = game_repository::get_game(id, db).await?;

        let actions = Action::read_many(id, db)
            .await?
            .iter()
            .map(|action| ActionDto {
                data: action.data.clone(),
                relic_id: action.relic_id,
                tick: action.tick,
                name: rows
                    .iter()
                    .find(|row| row.player_relic_id == action.relic_id)
                    .map(|p| p.player_name.clone())
                    .unwrap(),
            })
            .collect();
        let messages = Message::read_many(id, db)
            .await?
            .iter()
            .map(|message| MessageResponseDto {
                body: message.body.clone(),
                tick: message.tick,
                relic_id: message.player_relic_id,
                name: rows
                    .iter()
                    .find(|row| row.player_relic_id == message.player_relic_id)
                    .map(|p| p.player_name.clone())
                    .unwrap(),
            })
            .collect();

        let full_game = FullGameDto {
            actions,
            messages,
            ..rows.into()
        };

        Ok(full_game)
    }

    pub async fn create(game: Game, db: &Db) -> Result<(), PersistenceError> {
        game_repository::create_game(game, db).await?;

        Ok(())
    }

    pub async fn find_by_id(id: i32, db: &Db) -> Result<Option<Game>, PersistenceError> {
        let game = game_repository::find_game_by_id(id, db).await?;

        Ok(game)
    }
}
