use types::app_state::DbConfig;
use types::error::Error;
use types::quote::Quote;

#[tracing::instrument(skip(db_config))]
pub async fn get_quote(db_config: &DbConfig, author: &str) -> Result<Option<Quote>, Error> {
    tracing::info!("Fetching quote for {}", author);
    let try_quote: Option<Quote> = database::quote::get_random_quote(db_config, author).await?;
    Ok(try_quote)
}
