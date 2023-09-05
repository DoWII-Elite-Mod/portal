use calculate_ratings::Ratings;
use tracing::Level;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

mod calculate_ratings;

#[tokio::main]
pub async fn main() -> Result<(), database::error::PersistenceError> {
    let filter = filter::Targets::new()
        .with_target("sqlx::query", filter::LevelFilter::OFF)
        .with_target("ratings", Level::DEBUG);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let db = database::init_db().await?;
    Ratings::calculate(&db).await?;

    Ok(())
}
