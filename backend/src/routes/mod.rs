use std::sync::Arc;

use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig};
use actix_web::web;

use crate::middleware::rate_limit::ForwardedClientIpKeyExtractor;
use crate::module::health::handler::health_status;
use crate::module::newsletter::handler::subscribe;

pub fn config(
    cfg: &mut web::ServiceConfig,
    subscribe_governor: Arc<GovernorConfig<ForwardedClientIpKeyExtractor, NoOpMiddleware>>,
) {
    let g = subscribe_governor;
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_status))
            .service(
                web::resource("/subscribe")
                    .wrap(Governor::new(&*g))
                    .route(web::post().to(subscribe)),
            ),
    );
}
