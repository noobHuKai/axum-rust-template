use crate::error::ApiError;
use axum::extract::{Json, Query};
use axum_macros::{FromRequest, FromRequestParts};

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
pub struct AppJson<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(ApiError))]
pub struct AppQuery<T>(pub T);
