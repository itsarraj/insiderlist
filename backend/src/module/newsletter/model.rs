use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct SubscribeResponse {
    pub ok: bool,
    pub message: &'static str,
}
