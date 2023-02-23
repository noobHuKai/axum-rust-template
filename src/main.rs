use std::net::SocketAddr;
mod apis;
mod error;
mod extractor;
mod init;
mod middleware;
mod model;
mod route;
mod service;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ========================= init =========================
    // app init
    let cfg = init::init()?;
    // app state  init
    let app_state = state::AppState::init(&cfg).await?;

    // addr
    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));

    // server start
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(route::init_route(app_state).into_make_service())
        .with_graceful_shutdown(service::signal::shutdown_signal())
        .await?;

    Ok(())
}
