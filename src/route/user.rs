use crate::{apis::user, middleware::authorization::authorization, state::AppState};
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

// user route
// /users
pub fn init_user_route(state: AppState) -> Router<AppState> {
    Router::new().route("/login", post(user::login)).merge(
        // with midddle Authorization
        init_user_route_with_authorization()
            .layer(middleware::from_fn_with_state(state, authorization)),
    )
}

pub fn init_user_route_with_authorization() -> Router<AppState> {
    Router::new()
        .route("/", get(user::get_users))
        .route("/", post(user::create_user))
        .route("/", delete(user::delete_user))
}
