use crate::{middleware::log::log, state::AppState};
use axum::{middleware, Router};

mod user;

// root route
pub fn init_route(state: AppState) -> Router {
    Router::new()
        .nest("/users", user::init_user_route(state.clone()))
        .layer(middleware::from_fn(log))
        .with_state(state)
}
