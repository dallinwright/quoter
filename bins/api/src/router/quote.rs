use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use types::app_state::AppState;
use types::error::{Error};
use types::quote::Quote;

#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_quote_route(
    State(app_state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("routing get product: code={code}");

    // let try_data: Result<Quote, Error> = services::quote::get_quote(&app_state.db_pool, &code)
    //     .await;
    //
    // let data = try_data?;

    Ok((StatusCode::OK, Json("")))
}
