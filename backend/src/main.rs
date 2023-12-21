use axum::{
    routing::{get, post},
    Router, Server,
};
use log::info;
use simple_logger::SimpleLogger;
use sqlx::PgPool;

use crate::handlers::{fetch_all_data, store_env_data};

mod env_data;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    info!("Connecting to DB");
    let connection = PgPool::connect(env!("DATABASE_URL")).await.unwrap();

    let app = Router::new()
        .route("/", get(fetch_all_data))
        .route("/", post(store_env_data))
        .with_state(connection);

    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
