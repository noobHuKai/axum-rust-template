use crate::error::AppError;
use crate::extractor::AppJson;
use crate::model::{
    request::{CreateUserReq, LoginReq},
    response::LoginRes,
};
use crate::model::{ResponseMsg, UidReq};
use axum::Json;

use crate::service::token_service;
use crate::service::user::{self, insert_user, query_all_user, query_by_username_and_password};
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use redis::AsyncCommands;

// user login
pub async fn login(
    State(state): State<AppState>,
    AppJson(req): AppJson<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.pg_pool;
    let mut conn = state.redis_client.get_async_connection().await?;

    // query user
    let user = query_by_username_and_password(pool, req.username, req.password).await?;

    // generate token
    let token = token_service::generate_token();

    // set redis token
    let expire_time = state.config.token_expire_time;
    conn.set_ex(
        format!("token_{}", token),
        user.role.to_string(),
        expire_time,
    )
    .await?;

    Ok(Json(LoginRes {
        token,
        expires_in: expire_time,
    }))
}

pub async fn get_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let pool = &state.pg_pool;

    // query all user
    let users = query_all_user(pool).await?;

    Ok(Json(users))
}

pub async fn create_user(
    State(state): State<AppState>,
    AppJson(req): AppJson<CreateUserReq>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.pg_pool;

    // Insert user
    insert_user(pool, &req).await?;

    Ok(ResponseMsg::new("ok".to_string()).response())
}

pub async fn delete_user(
    State(state): State<AppState>,
    AppJson(req): AppJson<UidReq>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.pg_pool;

    // Insert user
    user::delete_user(pool, req.uid).await?;

    Ok(ResponseMsg::new("ok".to_string()).response())
}
