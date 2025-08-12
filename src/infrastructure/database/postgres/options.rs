#[derive(Clone, Debug)]
pub struct PostgresOptions {
    // Database name
    pub db: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub max_connections: u32
}

impl PostgresOptions {
    pub fn connection_url(&self) -> String {
        format!(
            "postgresql://{}@{}:{}/{}",
            self.user, self.password, self.host, self.port
        )
    }

    pub fn set_db(&mut self, postgres_db: &str) {
        self.db = postgres_db.to_owned();
    }

    pub fn set_max_connections(&mut self, max_connections: u32) {
        self.max_connections = max_connections;
    }

    pub const fn max_connections(&self) -> u32 {
        self.max_connections
    }
}
