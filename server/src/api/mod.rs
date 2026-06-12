use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::app::AppState;

mod health;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/health/ready", get(health::readiness))
        .fallback(not_found)
        .with_state(state)
}

async fn not_found() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::NOT_FOUND, "Not found".to_string())
}
