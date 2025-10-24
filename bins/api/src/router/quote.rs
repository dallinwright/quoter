use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use types::app_state::AppState;
use types::error::{Error};
use types::product::Product;

#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_quote_route(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("routing get product: code={code}");

    let try_product: Result<Product, Error> = services::product::get_product(&app_state.db_pool, &code)
        .await;

    let product = try_product?;

    Ok((StatusCode::OK, Json(product)))
}
