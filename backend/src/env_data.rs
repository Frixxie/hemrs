use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EnvDataEntry {
    pub ts: i64,
    pub room: String,
    pub temperature: f64,
    pub humidity: f64,
}

impl fmt::Display for EnvDataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "timestamp: {}, room: {}, temp: {}, hum: {}",
            self.ts, self.room, self.temperature, self.humidity
        )
    }
}

impl From<EnvData> for EnvDataEntry {
    fn from(env_data: EnvData) -> Self {
        EnvDataEntry {
            ts: Utc::now().timestamp(),
            room: env_data.room,
            temperature: env_data.temp,
            humidity: env_data.hum,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvData {
    room: String,
    temp: f64,
    hum: f64,
}

impl fmt::Display for EnvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.room, self.temp, self.hum,)
    }
}
