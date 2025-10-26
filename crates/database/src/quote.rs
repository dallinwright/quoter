use futures::StreamExt;
use tiberius::{AuthMethod, Client, Config, Row};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
use types::app_state::DbConfig;
use types::error::Error;
use types::quote::Quote;
use uuid::Uuid;

/// Establishes a TDS connection to Azure SQL using tiberius over Tokio TCP.
///
/// The connection is created for each call; consider using a pool for production workloads.
///
/// Parameters:
/// - `db_config`: Database connection settings (host, port, user, password, database name).
///
/// Returns:
/// - `Client<Compat<TcpStream)>`: An authenticated Tiberius client wrapped for Tokio I/O.
///
/// Panics:
/// - If DNS resolution or TCP connection fails.
/// - If TDS login fails.
///
/// Example:
/// ```rust
/// # async fn demo(cfg: types::app_state::DbConfig) {
/// let client = get_connection(&cfg).await;
/// // use `client` to run queries...
/// # }
/// ```
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

/// Inserts a new quote row with a generated UUID, applying session context for RLS.
///
/// Sets `sp_set_session_context` with the quote author for Row-Level Security,
/// then inserts into `dbo.quote (id, author, quote)`.
///
/// Parameters:
/// - `db_config`: Database connection settings.
/// - `quote`: The quote payload (author, quote text).
///
/// Returns:
/// - `Ok(())` on success.
/// - `Err(Error)` if the command execution fails.
///
/// Notes:
/// - Consider connection pooling for concurrent/throughput-heavy scenarios.
/// - Ensure the target table and RLS policy exist.
///
/// Example:
/// ```rust
/// # async fn demo(cfg: types::app_state::DbConfig) -> Result<(), types::error::Error> {
/// let q = types::quote::Quote {
///     id: uuid::Uuid::nil(), // ignored; new UUID is generated in the function
///     author: "Ada Lovelace".into(),
///     quote: "That brain of mine is something more than merely mortal.".into(),
/// };
/// insert_quote(&cfg, q).await?;
/// # Ok(())
/// # }
/// ```
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

/// Fetches a random quote for the given author using `TOP 1 ... ORDER BY NEWID()`.
///
/// Sets session context for RLS and returns one random row if available.
///
/// Parameters:
/// - `db_config`: Database connection settings.
/// - `author`: Author name used both for RLS context and filtering.
///
/// Returns:
/// - `Ok(Some(Quote))` if a quote exists for the author.
/// - `Ok(None)` if no quotes are found.
/// - `Err(Error)` if the query or row decoding fails.
///
/// Example:
/// ```rust
/// # async fn demo(cfg: types::app_state::DbConfig) -> Result<(), types::error::Error> {
/// if let Some(q) = get_random_quote(&cfg, "Ada Lovelace").await? {
///     assert_eq!(q.author, "Ada Lovelace");
/// }
/// # Ok(())
/// # }
/// ```
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


/// Retrieves a specific quote by UUID for the given author, honoring RLS.
///
/// Parameters:
/// - `db_config`: Database connection settings.
/// - `author`: Author name used for RLS context and filtering.
/// - `id`: Quote UUID.
///
/// Returns:
/// - `Ok(Some(Quote))` if the row exists and is visible under RLS.
/// - `Ok(None)` if not found.
/// - `Err(Error)` if the query or row decoding fails.
///
/// Example:
/// ```rust
/// # async fn demo(cfg: types::app_state::DbConfig, id: uuid::Uuid) -> Result<(), types::error::Error> {
/// if let Some(q) = get_quote_by_id(&cfg, "Ada Lovelace", id).await? {
///     assert_eq!(q.id, id);
/// }
/// # Ok(())
/// # }
/// ```
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
