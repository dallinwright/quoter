use std::env;
use std::sync::Arc;
use dotenvy::dotenv;
use types::app_state::{AppState, DbConfig};
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

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

    let db_user = safe_load_required_env_var("DATABASE_USER");
    let db_host = safe_load_required_env_var("DATABASE_HOST");
    let db_port = safe_load_required_env_var("DATABASE_PORT").parse::<u16>().expect("DATABASE_PORT must be set");
    let db_name = safe_load_required_env_var("DATABASE_NAME");
    let db_password = safe_load_required_env_var("DATABASE_PASSWORD");

    tracing::info!("Connected to database");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let db_config = DbConfig {
        user: db_user,
        host: db_host,
        port: db_port,
        password: db_password,
        name: db_name,
    };

    let state: AppState = AppState {
        db_config,
        port: port.parse().expect("PORT must be a number"),
    };

    tracing::info!("Application state loaded");
    state
}
