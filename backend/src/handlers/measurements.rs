use axum::{
    extract::{Query, State},
    Json,
};
use metrics::counter;
use sensors::Sensors;
use serde::Deserialize;
use tracing::{info, instrument, warn};

use crate::{
    database::{db_connection_pool::Postgres, insert::Insert, query::Query as MQuery, read::Read},
    measurements::Measurement,
};

use super::error::HandlerError;

#[instrument]
pub async fn store_measurements(
    State(pg_pool): State<Postgres>,
    Json(measurement): Json<Sensors>,
) -> Result<String, HandlerError> {
    match measurement {
        Sensors::Temperature(temperature) => {
            info!("Got temperature {}", temperature);
            temperature.insert(pg_pool).await.map_err(|e| {
                warn!("Failed with error: {}", e);
                HandlerError::new(500, format!("Failed to store data in database: {}", e))
            })?;
        }
        Sensors::Dht11(dht11) => {
            info!("Got dht11 {}", dht11);
            dht11.insert(pg_pool).await.map_err(|e| {
                warn!("Failed with error: {}", e);
                HandlerError::new(500, format!("Failed to store data in database: {}", e))
            })?;
        }
        Sensors::Measurement(measurement) => {
            info!("Got measurement {}", measurement);
            measurement.insert(pg_pool).await.map_err(|e| {
                warn!("Failed with error: {}", e);
                HandlerError::new(500, format!("Failed to store data in database: {}", e))
            })?;
        }
    };

    counter!("new_measurements").increment(1);

    Ok("OK".to_string())
}

#[instrument]
pub async fn fetch_measurements_count(
    State(pool): State<Postgres>,
) -> Result<Json<usize>, HandlerError> {
    let entry = Vec::<Measurement>::read(pool).await.map_err(|e| {
        warn!("Failed with error: {}", e);
        HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
    })?;

    Ok(Json(entry.len()))
}

#[derive(Debug, Deserialize, Clone)]
pub struct MeasurementByIdQuery {
    device_id: i32,
    sensor_id: i32,
}

#[instrument]
pub async fn fetch_measurements_by_ids(
    State(pool): State<Postgres>,
    Query(query): Query<MeasurementByIdQuery>,
) -> Result<Json<Vec<Measurement>>, HandlerError> {
    let entry = Measurement::read_measument_by_ids(query.device_id, query.sensor_id, pool)
        .await
        .map_err(|e| {
            warn!("Failed with error: {}", e);
            HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
        })?;

    Ok(Json(vec![entry]))
}

#[instrument]
pub async fn fetch_all_measurements(
    State(pool): State<Postgres>,
) -> Result<Json<Vec<Measurement>>, HandlerError> {
    let entry = Vec::<Measurement>::read(pool).await.map_err(|e| {
        warn!("Failed with error: {}", e);
        HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
    })?;

    Ok(Json(entry))
}
