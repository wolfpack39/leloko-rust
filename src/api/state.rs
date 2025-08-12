use crate::infrastructure::database::DatabasePool;
use std::sync::Arc;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub db_pool: DatabasePool
}