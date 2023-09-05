use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};
use tracing::info;

use self::error::PersistenceError;

pub mod error;
pub mod models;
pub mod repository;
pub mod types;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, PersistenceError> {
    let pool = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;

    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &Db) -> Result<(), PersistenceError> {
    // // Run migrations before starting the server
    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .map_err(|e| PersistenceError::Migrations(e.to_string()))
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let options: PgConnectOptions = PgConnectOptions::new()
        .host(host)
        .username(user)
        .password(pwd)
        .database(db);

    let pool = match std::env::var("DATABASE_URL") {
        Ok(url) => {
            PgPoolOptions::new()
                .max_connections(max_con)
                .connect(&url)
                .await?
        }
        Err(_) => {
            info!("Did not find DATABASE_URL. Defaulting to localhost.");
            PgPoolOptions::new()
                .max_connections(max_con)
                .connect_with(options)
                .await?
        }
    };

    Ok(pool)
}
