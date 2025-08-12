use sqlx::{PgConnection, PgPool};

use crate::infrastructure::database::postgres::{PostgresDatabase, PostgresOptions};

pub type DatabasePool = PgPool;
pub type DatabaseConnection = PgConnection;
pub type TestDatabase = PostgresDatabase;