use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use utoipa::OpenApi;

use crate::app::AppState;

mod health;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Irori API",
        description = "A shared hub for your memories and collections",
        version = "0.1.0",
        contact(
            name = "Irori",
            url = "https://github.com/saltyskip/irori"
        ),
        license(
            name = "MIT OR Apache-2.0"
        )
    ),
    paths(
        health::health,
        health::readiness
    )
)]
struct ApiDoc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/health/ready", get(health::readiness))
        .route("/openapi.json", get(openapi_spec))
        .fallback(not_found)
        .with_state(state)
}

/// OpenAPI specification endpoint
#[tracing::instrument(skip_all)]
async fn openapi_spec() -> impl IntoResponse {
    let spec = ApiDoc::openapi();
    Json(spec)
}

async fn not_found() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::NOT_FOUND, "Not found".to_string())
}
