
use std::sync::Arc;

use crate::infrastructure::DatabasePool;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub db_pool: DatabasePool
}