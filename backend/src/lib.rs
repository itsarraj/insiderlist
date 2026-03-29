use std::sync::Arc;

use actix_governor::GovernorConfigBuilder;
use actix_web::{App, HttpServer, middleware::Logger, web};

pub mod configuration;
pub mod middleware;
pub mod module;
pub mod routes;

use dotenvy::dotenv;
use middleware::rate_limit::ForwardedClientIpKeyExtractor;

pub type DbPool = sqlx::PgPool;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let configuration =
        configuration::get_configuration().expect("Failed to load configuration (configuration.yaml or APP__* env)");
    let database_url = configuration.database.connection_string();

    log::info!("Connecting to database at {}:{}", configuration.database.host, configuration.database.port);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let host = configuration.application_host.clone();
    let port = configuration.application_port;
    let email_service = crate::module::email::service::EmailService::new(&configuration.email);

    let rpm = configuration.rate_limit.subscribe_requests_per_minute.max(1);
    let burst = configuration.rate_limit.subscribe_burst.max(1);
    let subscribe_governor = Arc::new(
        GovernorConfigBuilder::default()
            .requests_per_minute(rpm)
            .burst_size(burst)
            .key_extractor(ForwardedClientIpKeyExtractor)
            .finish()
            .expect("subscribe rate limit: invalid governor config"),
    );
    log::info!("Subscribe rate limit: {} req/min per IP, burst {}", rpm, burst);

    log::info!("InsiderList API listening on {}:{}", host, port);

    HttpServer::new({
        let subscribe_governor = subscribe_governor.clone();
        move || {
            let gov = subscribe_governor.clone();
            App::new()
                .wrap(Logger::default())
                .wrap(middleware::cors::configure_cors())
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(email_service.clone()))
                .configure(move |c| routes::config(c, gov.clone()))
        }
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await?;

    Ok(())
}
