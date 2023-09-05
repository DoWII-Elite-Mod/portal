use crate::prelude::*;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use database::{
    models::games::types::{FullGameDto, Game, GameListItemDto, GamesQueryParameters},
    types::pageable::Pageable,
    Db,
};
use std::sync::Arc;

pub fn game_routes(db: &Db) -> Router {
    let db = Arc::new(db.clone());

    Router::new()
        .route("/", get(read_many))
        .route("/:id", get(read))
        .with_state(db)
}

async fn read(State(db): State<Arc<Db>>, Path(id): Path<i32>) -> Result<Json<FullGameDto>> {
    let game = Game::read(id, &db).await?;

    Ok(Json(game))
}

async fn read_many(
    State(db): State<Arc<Db>>,
    pagination: Query<GamesQueryParameters>,
) -> Result<Json<Pageable<GameListItemDto>>> {
    let games = Game::read_many(pagination.0, &db).await?;

    Ok(Json(games))
}
