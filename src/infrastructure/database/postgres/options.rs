#[derive(Clone, Debug)]
pub struct PostgresOptions {
    pub db: String,
    pub connection_url: String
}

impl PostgresOptions {
    pub fn connection_url(&self) -> String {
        format!("")
    }
}