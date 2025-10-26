use std::sync::Arc;

use crate::auth::clerk::ClerkJwks;
use crate::db::connection::DatabaseConnection;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub clerk_jwks: Arc<ClerkJwks>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, clerk_jwks: Arc<ClerkJwks>) -> Self {
        Self { db, clerk_jwks }
    }
}
