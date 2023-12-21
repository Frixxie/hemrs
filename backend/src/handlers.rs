use axum::{extract::State, Json};
use log::info;
use sqlx::{query, query_as, Pool, Postgres};

use crate::env_data::{EnvData, EnvDataEntry};

pub async fn store_env_data(
    State(pool): State<Pool<Postgres>>,
    Json(env_data): Json<EnvData>,
) -> String {
    info!("POST /");
    let env_data_entry: EnvDataEntry = env_data.into();
    query!(
        "INSERT INTO env_data VALUES ($1, $2, $3, $4)",
        env_data_entry.ts,
        env_data_entry.room,
        env_data_entry.temperature,
        env_data_entry.humidity
    )
    .execute(&pool)
    .await
    .unwrap();
    "OK".to_string()
}

pub async fn fetch_all_data(State(pool): State<Pool<Postgres>>) -> Json<Vec<EnvDataEntry>> {
    info!("GET /");
    let rows = query_as!(EnvDataEntry, "SELECT * FROM env_data")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}
