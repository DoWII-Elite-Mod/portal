use std::net::SocketAddr;

use axum::{extract::DefaultBodyLimit, Router};
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::{
    controllers::{game::game_routes, player::player_routes, reporter::reporter_routes},
    prelude::*,
};

mod controllers;
mod error;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::DEBUG)
        .with_target("tower_http::trace::on_request", Level::DEBUG)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_target("sqlx::query", filter::LevelFilter::OFF)
        .with_target("hyper::proto", filter::LevelFilter::OFF)
        .with_default(Level::DEBUG);

    let tracing_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    let pool = database::init_db()
        .await
        .map_err(|e| Error::Generic(format!("Database could not be initialized: {e}")))?;

    let app = Router::new()
        .nest("/players", player_routes(&pool))
        .nest("/games", game_routes(&pool))
        .nest("/reports", reporter_routes(&pool))
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
