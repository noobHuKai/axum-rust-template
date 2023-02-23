use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMsg {
    pub message: String,
}

impl ResponseMsg {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }

    pub fn response(self) -> impl IntoResponse {
        Json(self)
    }
}
