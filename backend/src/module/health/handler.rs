use std::time::Instant;

use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};

use crate::DbPool;
use crate::module::health::model::{ComponentHealth, HealthState, HealthStatus};
use crate::module::health::service::check_database;

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub async fn health_status(pool: web::Data<DbPool>) -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    let version = env!("CARGO_PKG_VERSION").to_string();
    let start_time = START_TIME.get_or_init(Instant::now);
    let uptime = start_time.elapsed().as_secs();

    let db_start = Instant::now();
    let database_status = match check_database(&pool).await {
        Ok(s) => s,
        Err(_) => HealthState::Unhealthy,
    };
    let database_ms = db_start.elapsed().as_millis() as u64;

    let components = vec![ComponentHealth::new(
        "database".to_string(),
        database_status.clone(),
        None,
        Some(database_ms),
        Some(now),
    )];

    let overall = if database_status == HealthState::Healthy {
        HealthState::Healthy
    } else {
        HealthState::Unhealthy
    };

    let health_status = HealthStatus {
        status: overall,
        timestamp: now,
        version,
        uptime,
        components,
    };

    let status = if overall == HealthState::Healthy {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };

    HttpResponse::build(status).json(&health_status)
}
