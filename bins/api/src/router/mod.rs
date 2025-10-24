use axum::Router;
use axum::routing::{get, post, put};
use types::app_state::AppState;
pub mod quote;

pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        // Product routes
        .route("/quote", get(get_quote_route))
        // attach shared state
        .with_state(app_state)
}
