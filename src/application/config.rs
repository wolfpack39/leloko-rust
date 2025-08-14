use std::env;

pub struct Config {
    // REST API configuration.
    pub service_host: String,
    pub service_port: u16,

    // PostgreSQL configuration.
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_connection_pool: u32,
}

impl Config {
    pub fn service_http_address(&self) -> String {
        format!("http://{}:{}", self.service_host, self.service_port)
    }

    pub fn postgres_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }

    pub fn load() -> Config {
        let env_file = if env_get_or("ENV_TEST", "0") == "1" {
          ".env_test"  
        } else {
            ".env"
        };

        let config = Config {
            service_host: env_get("SERVICE_HOST"),
            service_port: env_get("SERVICE_PORT"),
            postgres_user: env_get("POSTGRES_USER"),

        };

        config
    }
}

fn env_get(key: &str) -> String {
    match std::env::var(key) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("{} {}", key, e);
            tracing::error!(msg);
            panic!("{msg}");
        }
    }
}

fn env_get_or(key: &str, default: &str) -> String {
    if let Ok(v) = std::env::var(key) {
        return v;
    }
    default.to_owned()
}

fn env_parse() {
    
}