use crate::error::AuthError;
use crate::service::{casbin_service, token_service};
use crate::state::AppState;
use axum::extract::State;
use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use redis::AsyncCommands;

pub async fn authorization(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, AuthError> {
    // get token from head
    let token = token_service::get_bearer_token(&req)?;

    //  redis connection
    let mut conn = state
        .redis_client
        .get_async_connection()
        .await
        .map_err(|_| AuthError::InternalServerError)?;

    // get token and set expire time
    let role: String = conn
        .get_ex(
            format!("token_{}", token),
            redis::Expiry::EX(state.config.token_expire_time),
        )
        .await
        .map_err(|_| AuthError::InvalidToken)?;

    // check permissions
    // uri are change https://docs.rs/axum/0.6.7/axum/struct.Router.html#how-the-uri-changes
    casbin_service::check_permissions(
        state.enforcer,
        role,
        req.uri().path(),
        req.method().as_str(),
    )?;

    let res = next.run(req).await;

    Ok(res)
}
