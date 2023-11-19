use axum::{extract::State, Json};
use log::info;
use sqlx::{query, query_as, Pool, Sqlite};

use crate::env_data::{EnvData, EnvDataEntry};

pub async fn store_env_data(
    State(pool): State<Pool<Sqlite>>,
    Json(env_data): Json<EnvData>,
) -> String {
    info!("POST /");
    let env_data_entry: EnvDataEntry = env_data.into();
    query!(
        "INSERT INTO env_data (ts, room, temperature, humidity) VALUES (?, ?, ?, ?)",
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

pub async fn fetch_all_data(State(pool): State<Pool<Sqlite>>) -> Json<Vec<EnvDataEntry>> {
    info!("GET /");
    let rows = query_as!(EnvDataEntry, "SELECT * FROM env_data")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}
