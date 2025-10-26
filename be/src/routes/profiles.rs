use axum::{extract::{Extension, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::auth::middleware::AuthExtension;
use crate::app_state::AppState;

#[derive(Serialize, Deserialize)]
pub struct ProfileResponse {
    pub user_id: String,
    pub clerk_id: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

/// GET /api/v1/profiles/me - Get current user's profile
pub async fn get_current_profile(
    State(_app_state): State<AppState>,
    Extension(auth): Extension<AuthExtension>,
) -> Result<Json<ProfileResponse>, StatusCode> {
    let user = auth.0;

    Ok(Json(ProfileResponse {
        user_id: user.user_id,
        clerk_id: user.clerk_id,
        display_name: user.display_name,
        email: user.email,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub about_me: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProfileResponse {
    pub success: bool,
    pub message: String,
}

/// PUT /api/v1/profiles/me - Update current user's profile
pub async fn update_current_profile(
    State(_app_state): State<AppState>,
    Extension(auth): Extension<AuthExtension>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, StatusCode> {
    let user = auth.0;

    // Build UPDATE query
    let mut updates = Vec::new();

    if let Some(display_name) = payload.display_name {
        updates.push(format!("displayName = '{}'", display_name));
    }

    if let Some(about_me) = payload.about_me {
        updates.push(format!("aboutMe = '{}'", about_me));
    }

    if let Some(avatar_url) = payload.avatar_url {
        updates.push(format!("avatarUrl = '{}'", avatar_url));
    }

    if updates.is_empty() {
        return Ok(Json(UpdateProfileResponse {
            success: false,
            message: "No fields to update".to_string(),
        }));
    }

    updates.push("updatedAt = sysdate()".to_string());

    let query = format!(
        "UPDATE Profile SET {} WHERE userId = '{}'",
        updates.join(", "),
        user.user_id
    );

    // Execute via HTTP API
    let db_host = std::env::var("ARCADE_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = std::env::var("ARCADE_DB_PORT").unwrap_or_else(|_| "2480".to_string());
    let db_user = std::env::var("ARCADE_DB_USER").unwrap_or_else(|_| "root".to_string());
    let db_password = std::env::var("ARCADE_DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let db_name = std::env::var("ARCADE_DB_NAME").unwrap_or_else(|_| "cynnycty".to_string());

    let url = format!("http://{}:{}/api/v1/command/{}", db_host, db_port, db_name);
    let payload = serde_json::json!({
        "language": "sql",
        "command": query
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .basic_auth(&db_user, Some(&db_password))
        .json(&payload)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if response.status().is_success() {
        Ok(Json(UpdateProfileResponse {
            success: true,
            message: "Profile updated successfully".to_string(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
