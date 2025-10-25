use std::env;
use dotenvy::dotenv;
use types::app_state::AppState;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

fn safe_load_required_env_var(key: &str) -> String {
    env::var(key)
        .expect(&format!("Environment variable {} must be set", key))
        .trim_matches(|c| c == '"' || c == '\'')
        .to_string()
}

pub fn setup_baseline_telemetry(level: tracing::Level) {
    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(level.to_string())
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_writer(std::io::stdout)
        .try_init();
}

pub async fn load_app_state() -> AppState {
    dotenv().ok().unwrap_or_default();

    setup_baseline_telemetry(tracing::Level::INFO);

    tracing::info!("Connecting to database...");

    // Azure SQL connection details
    let mut config = Config::new();
    config.host("your-server-name.database.windows.net");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("your-username", "your-password"));
    config.trust_cert(); // Needed unless you validate certificates properly
    config.database("your-database-name");

    tracing::info!("Connected to database");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());


    let state: AppState = AppState {
        port: port.parse().expect("PORT must be a number"),
    };

    tracing::info!("Application state loaded");
    state
}
