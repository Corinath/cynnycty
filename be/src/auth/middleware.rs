use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::app_state::AppState;
use crate::auth::user::AuthUser;
use crate::db::connection::DatabaseConnection;

/// Extension type to store authenticated user in request
#[derive(Clone)]
pub struct AuthExtension(pub AuthUser);

/// Middleware to verify Clerk JWT and lookup/create user profile
pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check for "Bearer <token>" format
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the JWT with Clerk
    let claims = app_state.clerk_jwks
        .verify_token(token)
        .await
        .map_err(|e| {
            tracing::error!("JWT verification failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    let clerk_id = claims.sub;
    tracing::debug!("Authenticated Clerk user: {}", clerk_id);

    // Look up or create the user profile
    let user = lookup_or_create_profile(&app_state.db, &clerk_id, &claims.email, &claims.name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to lookup/create profile: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Insert the authenticated user into request extensions
    request.extensions_mut().insert(AuthExtension(user));

    Ok(next.run(request).await)
}

/// Look up a profile by clerkId, or create a new one if it doesn't exist
async fn lookup_or_create_profile(
    db: &DatabaseConnection,
    clerk_id: &str,
    email: &Option<String>,
    name: &Option<String>,
) -> Result<AuthUser, Box<dyn std::error::Error>> {
    let database = db.get_db();

    // Try to find existing profile by clerkId
    let query = format!("SELECT FROM Profile WHERE clerkId = '{}'", clerk_id);

    match database.query(&query).send::<serde_json::Value>().await {
        Ok(results) if !results.is_empty() => {
            // Profile exists, extract userId
            let profile = &results[0];
            let user_id = profile["userId"]
                .as_str()
                .ok_or("Missing userId in profile")?
                .to_string();

            tracing::info!("Found existing profile: userId={}", user_id);

            Ok(AuthUser::new(clerk_id.to_string(), user_id)
                .with_email(email.clone())
                .with_display_name(name.clone()))
        }
        _ => {
            // Profile doesn't exist, create a new one
            let user_id = uuid::Uuid::new_v4().to_string();
            let display_name = name.as_ref().unwrap_or(&"User".to_string()).clone();

            tracing::info!(
                "Creating new profile: userId={}, clerkId={}",
                user_id,
                clerk_id
            );

            // Create the profile
            let create_query = format!(
                "INSERT INTO Profile SET userId = '{}', clerkId = '{}', displayName = '{}', createdAt = sysdate(), updatedAt = sysdate()",
                user_id, clerk_id, display_name
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
                "command": create_query
            });

            let client = reqwest::Client::new();
            client
                .post(&url)
                .basic_auth(&db_user, Some(&db_password))
                .json(&payload)
                .send()
                .await?;

            tracing::info!("Profile created successfully");

            Ok(AuthUser::new(clerk_id.to_string(), user_id)
                .with_email(email.clone())
                .with_display_name(Some(display_name)))
        }
    }
}
