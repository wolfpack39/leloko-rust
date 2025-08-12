use sqlx::PgPool;

use crate::infrastructure::database::{postgres::PostgresDatabase, PostgresOptions};

pub type DatabasePool = PgPool;

#[derive(Clone, Debug)]
pub struct DatabaseOptions {
    pub postgres: PostgresOptions
}

pub struct Database;

impl Database {
    pub async fn connect(options: DatabaseOptions) -> Result<DatabasePool, sqlx::Error>{
        let db = PostgresDatabase::connect(options).await?;
        Ok(db.pool().clone())
    }
}