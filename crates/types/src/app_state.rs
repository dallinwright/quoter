#[derive(Clone)]
pub struct AppState {
    pub port: u16,
    pub db_config: DbConfig
}

#[derive(Clone)]
pub struct DbConfig {
    pub user: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub password: String,
}
