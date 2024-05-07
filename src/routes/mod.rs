use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::get,
    Router,
};
use tower_http::{cors::{Any, CorsLayer}, trace:: TraceLayer};

use crate::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let tracing_layer = TraceLayer::new_for_http();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", get(|| async { "Phantom vasploit" }))
        .with_state(state)
        .layer(cors)
        .layer(tracing_layer)
}
