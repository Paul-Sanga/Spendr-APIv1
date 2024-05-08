pub mod users;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use self::users::{authectication::{login, register_user}, user_management::get_users};
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
        .route("/", get(|| async { "Phantom Vasploit" }))
        .route("/api/v1/auth/register", post(register_user))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/users", get(get_users))
        .layer(cors)
        .layer(tracing_layer)
        .with_state(state)
}
