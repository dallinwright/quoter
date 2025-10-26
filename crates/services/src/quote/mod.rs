use uuid::Uuid;
use types::app_state::DbConfig;
use types::error::Error;
use types::quote::Quote;

// The purpose is to show separation in layers between the routing layer, the business logic "service" layer and the database itself.
// Allows swapping/switching and centralized management of code.

#[tracing::instrument(skip(db_config))]
pub async fn get_quote(db_config: &DbConfig, author: &str) -> Result<Option<Quote>, Error> {
    tracing::info!("Fetching quote for {}", author);
    let try_quote: Option<Quote> = database::quote::get_random_quote(db_config, author).await?;
    Ok(try_quote)
}

#[tracing::instrument(skip(db_config))]
pub async fn get_quote_by_id(db_config: &DbConfig, author: &str, id: Uuid) -> Result<Option<Quote>, Error> {
    tracing::info!("Fetching quote for {}", author);
    let try_quote: Option<Quote> = database::quote::get_quote_by_id(db_config, author, id).await?;
    Ok(try_quote)
}
