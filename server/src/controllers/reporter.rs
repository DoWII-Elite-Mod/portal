use crate::prelude::*;
use std::sync::Arc;

use database::{models::replay_report::ReplayReportDto, Db};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

pub fn reporter_routes(pool: &Db) -> Router {
    let pool = Arc::new(pool.clone());
    Router::new().route("/", post(save)).with_state(pool)
}

async fn save(
    State(db): State<Arc<Db>>,
    Json(replay): Json<ReplayReportDto>,
) -> Result<impl IntoResponse> {
    ReplayReportDto::create(&db, replay).await?;

    Ok((StatusCode::CREATED, ()))
}
