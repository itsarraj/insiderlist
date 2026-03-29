use actix_web::{HttpResponse, Responder, web};
use regex::Regex;
use std::sync::LazyLock;

use crate::module::email::service::EmailService;
use crate::module::newsletter::model::{SubscribeRequest, SubscribeResponse};
use crate::DbPool;

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,64}$").expect("email regex")
});

pub async fn subscribe(
    pool: web::Data<DbPool>,
    email_service: web::Data<EmailService>,
    body: web::Json<SubscribeRequest>,
) -> impl Responder {
    let email = body.email.trim();
    if email.is_empty() || email.len() > 320 || !EMAIL_RE.is_match(email) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "ok": false,
            "message": "Please enter a valid email address."
        }));
    }

    let email_normalized = email.to_lowercase();

    let inserted = match sqlx::query_scalar::<_, uuid::Uuid>(
        r#"
        INSERT INTO newsletter_subscribers (email)
        VALUES ($1)
        ON CONFLICT (email) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(&email_normalized)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("subscribe db error: {e}");
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "ok": false,
                "message": "Something went wrong. Please try again."
            }));
        }
    };

    if inserted.is_some() {
        if let Err(e) = email_service
            .send_waitlist_confirmation(&email_normalized)
            .await
        {
            log::warn!("Could not send welcome email: {e}");
        }
    }

    HttpResponse::Ok().json(SubscribeResponse {
        ok: true,
        message: "Thanks — we'll be in touch.",
    })
}
