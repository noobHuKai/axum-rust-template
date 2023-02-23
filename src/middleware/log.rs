use crate::error::AppError;
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};

pub async fn log(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse, AppError> {
    tracing::info!(
        target:"access",
        "{} {}",
        req.method(),
        req.uri().path(),
    );
    let res = next.run(req).await;

    Ok(res)
}
