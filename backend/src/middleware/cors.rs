use actix_cors::Cors;
use actix_web::http::header;

pub fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
        .max_age(3600)
}
