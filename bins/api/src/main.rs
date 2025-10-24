use crate::config::load_app_state;

use crate::router::build_router;
use axum::Router;
use types::app_state::AppState;

mod config;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state: AppState = load_app_state().await;
    let port = app_state.port;

    let app: Router = build_router(app_state);

    let address: String = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Failed to start server");

    Ok(())
}
