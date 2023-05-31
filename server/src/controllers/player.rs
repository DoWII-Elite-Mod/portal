use crate::prelude::*;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use database::{
    models::{player::Player, player_details::PlayerDetailsDto},
    types::pagination::Pagination,
    Db,
};

pub fn player_routes(db: &Db) -> Router {
    let db = Arc::new(db.clone());
    Router::new()
        .route("/", get(read_many))
        .route("/", post(create))
        .route("/:id", get(read))
        .with_state(db)
}

async fn read(Path(id): Path<i32>, State(db): State<Arc<Db>>) -> Result<Json<PlayerDetailsDto>> {
    let player = Player::get_details(id, &db).await?;

    Ok(Json(player))
}

async fn read_many(
    Query(pagination): Query<Pagination>,
    State(db): State<Arc<Db>>,
) -> Result<Json<Vec<Player>>> {
    let players = Player::read_many(pagination, &db).await?;

    Ok(Json(players))
}

async fn create(
    State(db): State<Arc<Db>>,
    Json(player): Json<Player>,
) -> Result<impl IntoResponse> {
    Player::create(player, &db).await?;

    Ok((StatusCode::CREATED, ()))
}
