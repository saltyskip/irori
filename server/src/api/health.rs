use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::app::AppState;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "0.1.0")]
    pub version: String,
}

/// Health check endpoint
///
/// Simple health check that always returns OK. Use this to check if the server is running.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is healthy", body = HealthResponse)
    )
)]
#[tracing::instrument(skip_all)]
pub async fn health(State(_state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Readiness check (includes database)
///
/// Checks if the server is ready to handle requests. Verifies database connectivity.
#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = 200, description = "Server is ready", body = HealthResponse),
        (status = 503, description = "Service unavailable (database not connected)")
    )
)]
#[tracing::instrument(skip_all)]
pub async fn readiness(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>, StatusCode> {
    // Try to check database connection
    sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(HealthResponse {
        status: "ready".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
