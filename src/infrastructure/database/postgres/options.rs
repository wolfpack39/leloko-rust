#[derive(Clone, Debug)]
pub struct PostgresOptions {
    pub db: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub max_connections: u16,
}

impl PostgresOptions {
    pub fn connection_url(&self) -> String {
        format!("")
    }
}