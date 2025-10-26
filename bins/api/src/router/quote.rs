use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use uuid::Uuid;
use types::app_state::AppState;
use types::error::{Error};
use types::quote::Quote;


#[derive(Serialize)]
struct ApiError {
    message: String,
}

#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_quote_route(
    State(app_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("routing get quote");

    // This is a stub. You would validate user data typically via Oauth2 via JWT
    // then use that user id to validate access, claims, etc. This is to sidestep that but show how it would
    // work. The header would come from a middleware extractor or authed in the infra level.
    let fake_user_id = headers.get("X-User-Id");
    let Some(author) = fake_user_id else {
        return Ok((StatusCode::UNAUTHORIZED, Json(ApiError { message: "not authorized".to_string() })).into_response())
    };

    let parsed_author = author.to_str()
        .map_err(|_| Error::internal("invalid header"))?;

    let try_data: Option<Quote> = services::quote::get_quote(&app_state.db_config, parsed_author).await
        .map_err(|err| {
            tracing::error!("failed to query for quote: {:?}", err);
            Error::internal("failed")
        })?;

    let resp = match try_data {
        Some(data) => (StatusCode::OK, Json(data)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(ApiError { message: "not found".to_string() })).into_response()
    };

    Ok(resp)
}

#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_quote_by_id_route(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,            // <- extract id from path
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, Error> {
    tracing::debug!("routing get quote");

    // This is a stub. You would validate user data typically via Oauth2 via JWT
    // then use that user id to validate access, claims, etc. This is to sidestep that but show how it would
    // work. The header would come from a middleware extractor or authed in the infra level.
    let fake_user_id = headers.get("X-User-Id");
    let Some(author) = fake_user_id else {
        return Ok((StatusCode::UNAUTHORIZED, Json(ApiError { message: "not authorized".to_string() })).into_response())
    };

    let parsed_author = author.to_str()
        .map_err(|_| Error::internal("invalid header"))?;

    let try_data: Option<Quote> = services::quote::get_quote_by_id(&app_state.db_config, parsed_author, id).await
        .map_err(|err| {
            tracing::error!("failed to query for quote: {:?}", err);
            Error::internal("failed")
        })?;

    let resp = match try_data {
        Some(data) => (StatusCode::OK, Json(data)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(ApiError { message: "not found".to_string() })).into_response()
    };

    Ok(resp)
}
