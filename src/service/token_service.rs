use crate::error::AuthError;
use axum::{body::Body, http::Request};
use http::header::AUTHORIZATION;

pub fn get_bearer_token(req: &Request<Body>) -> Result<&str, AuthError> {
    // get head AUTHORIZATION Field
    let auth_str = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthError::MissingCredentials)?
        .to_str()
        .map_err(|_| AuthError::WrongCredentials)?;

    // get Bearer token
    let token = match auth_str.split_once(' ') {
        // Found proper bearer
        Some((name, content)) if name == "Bearer" => content,
        // Found nothing
        _ => return Err(AuthError::TokenCreation),
    };

    Ok(token)
}

pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}
