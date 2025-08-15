
use std::sync::Arc;

use crate::{application::config::Config, infrastructure::DatabasePool};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub db_pool: DatabasePool,
    pub config: Config,
}