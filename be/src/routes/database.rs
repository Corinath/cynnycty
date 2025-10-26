use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::db::connection::DatabaseConnection;

#[derive(Serialize, Deserialize)]
pub struct DatabaseHealthResponse {
    pub status: String,
    pub database: String,
    pub connected: bool,
    pub message: String,
}

pub async fn database_health_check(
    State(db): State<DatabaseConnection>,
) -> Result<Json<DatabaseHealthResponse>, StatusCode> {
    match db.health_check().await {
        Ok(_) => Ok(Json(DatabaseHealthResponse {
            status: "ok".to_string(),
            database: db.database_name.clone(),
            connected: true,
            message: "Database connection is healthy".to_string(),
        })),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
