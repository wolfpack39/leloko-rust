use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::infrastructure::database::DatabaseOptions;

#[non_exhaustive]
pub struct PostgresDatabase {
    pool: PgPool,
    options: DatabaseOptions
}

impl PostgresDatabase {
    pub async fn connect(options: DatabaseOptions) -> Result<Self, sqlx::Error>{
        let connection_url = options.postgres.connection_url();
        let pool = PgPoolOptions::new()
            .connect(&connection_url)
            .await?;
        Ok(Self { pool, options })
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }
}