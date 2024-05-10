pub mod users;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use self::users::{
    authectication::{login, register_user},
    user_management::{delete_user, get_user_by_id, get_users, update_user},
};
use crate::{app_state::AppState, middleware::auth_middleware::require_authentication};

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
        .route("/api/v1/users", get(get_users))
        .route("/api/v1/users/:id", get(get_user_by_id))
        .route("/api/v1/users/:id", put(update_user))
        .route("/api/v1/users/:id", delete(delete_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .route("/api/v1/auth/register", post(register_user))
        .route("/api/v1/auth/login", post(login))
        .layer(cors)
        .layer(tracing_layer)
        .with_state(state)
}
