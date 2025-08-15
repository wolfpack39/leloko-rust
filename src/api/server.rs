use std::sync::Arc;

use axum::{
    Json, 
    Router, 
    middleware::{self, Next},
    body::Body,
    http::{HeaderMap, StatusCode, Method},
    response::{IntoResponse, Response},
    routing::{any, get}
};

use tokio::{
    net::TcpListener,
    signal::{
        self, 
        unix::{self, SignalKind}
    }
};

use tower_http::cors::{Any, Corslayer};
use crate::application::state::SharedState;

pub async fn start(state: SharedState) {
    let cors_layer = Corslayer::new().allow_origin(Any);

    let router = Router::new()
        .route("path", get(root_handler))
        .with_state(Arc::clone(&state))
        .layer(cors_layer)
        .layer(middleware::from_fn(logging_middleware));
}