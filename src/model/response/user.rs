use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginRes {
    pub token: String,
    pub expires_in: usize,
}
