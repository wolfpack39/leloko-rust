use sqlx::{
    postgres::PgPoolOptions, PgPool
};

use crate::infrastructure::database::postgres::options::PostgresOptions;

#[non_exhaustive]
pub struct PostgresDatabase {
    pool: PgPool,
    options: PostgresOptions,
    test_db_to_drop: Option<String>
}

impl PostgresDatabase {
    pub async fn connect(options: PostgresOptions) -> Result<Self, sqlx::Error> {
        let connection_url = options.connection_url();
        let max_connections = options.max_connections();

        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&connection_url)
            .await?;

        tracing::info!("Connected to PostgreSQL Database");

        Ok(Self { pool, options, test_db_to_drop: None })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub  async fn drop(&self) -> Result<(), sqlx::Error> {
        if let Some(test_db_to_drop) = self.test_db_to_drop.as_ref() {
            self.pool.close().await;

            let db = Self::connect(self.options.clone()).await?;
            let pool = db.pool();
            let query = format!("DROP DATABASE IF EXISTS {}", test_db_to_drop);
            sqlx::query(&query).execute(pool).await?;
        }
        Ok(())
    }

}