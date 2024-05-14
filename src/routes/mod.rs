pub mod budget;
pub mod tip;
pub mod transactions;
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

use self::{
    budget::budget_management::{
        create_budget, delete_budget, get_budget, get_budget_by_id, update_budget,
    },
    tip::tip_management::{create_tip, get_tip_by_id},
    transactions::transaction_management::{
        create_transaction, delete_transaction, get_transaction_by_id, get_transactions,
        update_transaction,
    },
    users::{
        authectication::{login, register_user},
        user_management::{delete_user, get_user_by_id, get_users, update_user},
    },
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
        .route("/api/v1/budget", post(create_budget))
        .route("/api/v1/budget/:id", get(get_budget_by_id))
        .route("/api/v1/budget", get(get_budget))
        .route("/api/v1/budget/:id", put(update_budget))
        .route("/api/v1/budget/:id", delete(delete_budget))
        .route("/api/v1/transactions", post(create_transaction))
        .route("/api/v1/transactions", get(get_transactions))
        .route("/api/v1/transactions/:id", get(get_transaction_by_id))
        .route("/api/v1/transactions/:id", put(update_transaction))
        .route("/api/v1/transactions/:id", delete(delete_transaction))
        .route("/api/v1/tip", post(create_tip))
        .route("/api/v1/tip/:id", get(get_tip_by_id))
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
