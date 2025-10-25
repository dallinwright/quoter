use futures::StreamExt;
use tiberius::{AuthMethod, Client, Config, Row};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use types::app_state::DbConfig;
use types::error::Error;
use types::quote::Quote;
use uuid::Uuid;

// In theory use a connection pool or something like PgBouncer but for mssql.
pub async fn get_connection(db_config: &DbConfig) -> Client<Compat<TcpStream>> {
    // Azure SQL connection details
    let mut config = Config::new();
    config.host(db_config.host.clone());
    config.port(db_config.port);
    config.authentication(AuthMethod::sql_server(db_config.user.clone(), db_config.password.clone()));
    config.trust_cert(); // Needed unless you validate certificates properly
    config.database(db_config.name.clone());

    let tcp = TcpStream::connect(config.get_addr()).await.expect("Can't connect to server");
    let client: Client<Compat<TcpStream>> = Client::connect(config, tcp.compat_write()).await.expect("Can't connect to server");
    client
}

pub async fn insert_quote(db_config: &DbConfig, quote: Quote) -> Result<(), Error> {
    // Lock the client for this operation
    // This is potentially problematic
    let mut client = get_connection(db_config).await;

    // Get session context for RLS
    client.execute(
        "EXEC sp_set_session_context @key=N'user_name', @value=@P1",
        &[&quote.author],
    ).await?;

    // Insert a row
    let new_id = Uuid::new_v4();
    client.execute(
        "INSERT INTO dbo.quote (id, author, quote) VALUES (@P1, @P2, @P3)",
        &[&new_id, &quote.author, &quote.quote],
    ).await?;

    Ok(())
}

/// Select a random quote for a given author
pub async fn get_random_quote(db_config: &DbConfig, author: &str) -> Result<Option<Quote>, Error> {
    let mut client = get_connection(db_config).await;

    // Set session context for RLS
    client.execute(
        "EXEC sp_set_session_context @key=N'user_name', @value=@P1",
        &[&author],
    ).await?;

    // Query a random quote (TOP 1 ORDER BY NEWID())
    let stream = client
        .query(
            "SELECT TOP 1 id, quote, author FROM dbo.quote WHERE author = @P1 ORDER BY NEWID()",
            &[&author],
        )
        .await?;

    // Convert the QueryStream into only rows, skipping metadata
    let mut row_stream = stream.into_row_stream();

    if let Some(row) = row_stream.next().await {
        let row: Row = row?;
        let id: Uuid = row.get(0).expect("Couldn't get id");
        let quote_text: &str = row.get(1).expect("Couldn't get quote_text");
        let author: &str = row.get(2).expect("Couldn't get author");

        Ok(Some(Quote {
            id,
            author: author.to_string(),
            quote: quote_text.to_string(),
        }))
    } else {
        Ok(None) // no quotes found for this author
    }
}


/// Select a quote by id for a given author
pub async fn get_quote_by_id(
    db_config: &DbConfig,
    author: &str,
    id: Uuid,
) -> Result<Option<Quote>, Error> {
    // Get a connection
    let mut client = get_connection(db_config).await;

    // Set session context for RLS (optional)
    client
        .execute(
            "EXEC sp_set_session_context @key=N'user_name', @value=@P1",
            &[&author],
        )
        .await?;

    // Query quote by ID
    let stream = client
        .query(
            "SELECT id, quote, author FROM dbo.quote WHERE author = @P1 AND id = @P2",
            &[&author, &id],
        )
        .await?;

    let mut row_stream = stream.into_row_stream();

    if let Some(row) = row_stream.next().await {
        let row: Row = row?;
        let id: Uuid = row.get("id").expect("Couldn't get id");
        let quote_text: &str = row.get("quote").expect("Couldn't get quote");
        let author: &str = row.get("author").expect("Couldn't get author");

        Ok(Some(Quote {
            id,
            author: author.to_string(),
            quote: quote_text.to_string(),
        }))
    } else {
        Ok(None) // no quote found
    }
}
