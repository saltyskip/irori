use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::Scalar;

use crate::app::AppState;

mod health;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Hearth API",
        description = "A shared hub for your memories and collections",
        version = "0.1.0"
    ),
    paths(),
    components()
)]
struct ApiDoc;

pub fn router(state: Arc<AppState>) -> Router {
    let health_router = Router::new()
        .route("/", get(health::health))
        .route("/ready", get(health::readiness))
        .with_state(state.clone());

    let api_router = Router::new()
        .nest("/health", health_router)
        .with_state(state);

    // OpenAPI documentation
    let (mut api_router, api_doc) = OpenApiRouter::new()
        .routes(api_router.routes.routes)
        .split_for_scalar();

    api_router = api_router
        .merge(Scalar::with_url("/docs", api_doc))
        .fallback(axum::routing::get(not_found));

    api_router
}

async fn not_found() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::NOT_FOUND, "Not found".to_string())
}
