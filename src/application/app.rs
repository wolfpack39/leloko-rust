use std::sync::Arc;

use crate::{
    api::server,
    application::{config, state::AppState},
    infrastructure::database::Database
};

pub async fn run() {
    let config = config::load();

    let db_pool = Database::connect(config.clone().into())
        .await
        .expect("Failed to connect to the database");

    // Build the application state
    let shared_state = Arc::new(AppState{
        config,
        db_pool
    });

    server::start(shared_state).await;
}
