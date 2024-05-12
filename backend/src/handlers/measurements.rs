use axum::{
    extract::{Query, State},
    Json,
};
use log::{info, warn};
use sensors::Sensors;
use serde::Deserialize;

use crate::{
    database::{db_connection_pool::Postgres, insert::Insert, query::Query as MQuery, read::Read},
    measurements::Measurement,
};

use super::error::HandlerError;

pub async fn store_measurements(
    State(pg_pool): State<Postgres>,
    Json(measurement): Json<Sensors>,
) -> Result<String, HandlerError> {
    info!("POST /");

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

    Ok("OK".to_string())
}

#[derive(Deserialize)]
pub struct MeasurementQuery {
    pub device_name: String,
    pub sensor_name: String,
}

pub async fn fetch_latest_measurement(
    State(pg_pool): State<Postgres>,
    Query(measurements_query): Query<Option<MeasurementQuery>>,
) -> Result<Json<Measurement>, HandlerError> {
    info!("GET api/measurements/latest");

    let entry = match measurements_query {
        Some(query) => query.query(pg_pool).await.map_err(|e| {
            warn!("Failed with error: {}", e);
            HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
        })?,
        None => Measurement::read(pg_pool).await.map_err(|e| {
            warn!("Failed with error: {}", e);
            HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
        })?,
    };

    Ok(Json(entry))
}

pub async fn fetch_all_measurements(
    State(pool): State<Postgres>,
) -> Result<Json<Vec<Measurement>>, HandlerError> {
    info!("GET api/measurements");
    let entry = Vec::<Measurement>::read(pool).await.map_err(|e| {
        warn!("Failed with error: {}", e);
        HandlerError::new(500, format!("Failed to fetch data from database: {}", e))
    })?;

    Ok(Json(entry))
}
