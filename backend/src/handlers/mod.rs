use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post, put},
    Router,
};
use metrics::histogram;
use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::Pool;
use tokio::time::Instant;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

use crate::database::db_connection_pool;

use self::{
    devices::{
        delete_device, fetch_devices, fetch_measurement_by_device_id,
        fetch_measurement_by_device_id_and_sensor_id, insert_device, update_device,
    },
    measurements::{
        fetch_all_measurements, fetch_latest_measurement, fetch_measurements_count,
        store_measurements,
    },
    sensors::{delete_sensor, fetch_sensors, insert_sensor, update_sensor},
};

pub use measurements::InnerMeasurementQuery;

mod devices;
mod error;
mod measurements;
mod sensors;

#[instrument]
pub async fn profile_endpoint(request: Request, next: Next) -> Response {
    let method = request.method().clone().to_string();
    let uri = request.uri().clone().to_string();
    info!("Handling {} at {}", method, uri);

    let now = Instant::now();

    let labels = [("method", method.clone()), ("uri", uri.clone())];

    let response = next.run(request).await;

    let elapsed = now.elapsed();

    histogram!("handler", &labels).record(elapsed);

    info!(
        "Finished handling {} at {}, used {} ms",
        method,
        uri,
        elapsed.as_millis()
    );
    response
}

pub fn create_router(
    connection: Pool<sqlx::Postgres>,
    metrics_handler: PrometheusHandle,
) -> Router {
    let pg_pool = db_connection_pool::Postgres::new(connection);

    let measurements = Router::new()
        .route("/measurements", get(fetch_all_measurements))
        .route("/measurements/latest", get(fetch_latest_measurement))
        .route("/measurements/count", get(fetch_measurements_count))
        .route("/measurements", post(store_measurements));

    let devices = Router::new()
        .route("/devices", get(fetch_devices))
        .route("/devices", post(insert_device))
        .route("/devices", delete(delete_device))
        .route("/devices", put(update_device))
        .route(
            "/devices/:device_id/measurements",
            get(fetch_measurement_by_device_id),
        )
        .route(
            "/devices/:device_id/sensors/:sensor_id/measurements",
            get(fetch_measurement_by_device_id_and_sensor_id),
        );

    let sensors = Router::new()
        .route("/sensors", get(fetch_sensors))
        .route("/sensors", post(insert_sensor))
        .route("/sensors", delete(delete_sensor))
        .route("/sensors", put(update_sensor));

    Router::new()
        .nest("/api", measurements)
        .nest("/api", devices)
        .nest("/api", sensors)
        .route("/", post(store_measurements))
        .with_state(pg_pool)
        .route("/metrics", get(metrics))
        .with_state(metrics_handler)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(profile_endpoint)),
        )
}

#[instrument]
async fn metrics(State(handle): State<PrometheusHandle>) -> String {
    handle.render()
}
