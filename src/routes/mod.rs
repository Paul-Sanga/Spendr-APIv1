pub mod users;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get,post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::app_state::AppState;
use self::users::authectication::register_user;

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
        .route("/", get(|| async { "Phantom Vasploit" }))
        .route("/api/v1/user/register", post(register_user))
        .layer(cors)
        .layer(tracing_layer)
        .with_state(state)
}
