use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_governor::governor::clock::{Clock, DefaultClock, QuantaInstant};
use actix_governor::governor::NotUntil;
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::dev::ServiceRequest;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, HttpResponseBuilder};

/// Uses the first IP in `X-Forwarded-For`, then `X-Real-IP`, then the socket peer.
/// Unknown clients share the `0.0.0.0` bucket so they are still limited.
#[derive(Clone, Debug, Eq)]
pub struct ClientIpKey(pub IpAddr);

impl PartialEq for ClientIpKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for ClientIpKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct ForwardedClientIpKeyExtractor;

fn parse_client_ip(req: &ServiceRequest) -> ClientIpKey {
    if let Some(ff) = req.headers().get("x-forwarded-for").and_then(|h| h.to_str().ok()) {
        if let Some(first) = ff.split(',').next() {
            if let Ok(ip) = first.trim().parse::<IpAddr>() {
                return ClientIpKey(ip);
            }
        }
    }
    if let Some(real) = req.headers().get("x-real-ip").and_then(|h| h.to_str().ok()) {
        if let Ok(ip) = real.trim().parse::<IpAddr>() {
            return ClientIpKey(ip);
        }
    }
    if let Some(peer) = req.connection_info().peer_addr() {
        if let Ok(addr) = peer.parse::<SocketAddr>() {
            return ClientIpKey(addr.ip());
        }
        if let Ok(ip) = peer.parse::<IpAddr>() {
            return ClientIpKey(ip);
        }
    }
    ClientIpKey(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
}

impl KeyExtractor for ForwardedClientIpKeyExtractor {
    type Key = ClientIpKey;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        Ok(parse_client_ip(req))
    }

    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut response: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait_secs = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs()
            .max(1);
        response
            .insert_header(("Retry-After", wait_secs.to_string()))
            .content_type(ContentType::json())
            .body(format!(
                r#"{{"ok":false,"message":"Too many sign-up attempts from this network. Try again in about {} seconds."}}"#,
                wait_secs
            ))
    }
}
