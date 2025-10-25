use axum::routing::get;
use axum::Router;
use types::app_state::AppState;
pub mod quote;

use crate::router::quote::{get_quote_by_id_route, get_quote_route};

pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        // Product routes
        .route("/quote", get(get_quote_route))
        .route("/quote/{id}", get(get_quote_by_id_route))
        // attach shared state
        .with_state(app_state)
}
