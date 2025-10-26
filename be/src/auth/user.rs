use serde::{Deserialize, Serialize};

/// Authenticated user with both clerkId and internal userId
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    /// Clerk's user ID
    pub clerk_id: String,
    /// Our internal user ID (from Profile table)
    pub user_id: String,
    /// User's email
    pub email: Option<String>,
    /// User's display name
    pub display_name: Option<String>,
}

impl AuthUser {
    pub fn new(clerk_id: String, user_id: String) -> Self {
        Self {
            clerk_id,
            user_id,
            email: None,
            display_name: None,
        }
    }

    pub fn with_email(mut self, email: Option<String>) -> Self {
        self.email = email;
        self
    }

    pub fn with_display_name(mut self, display_name: Option<String>) -> Self {
        self.display_name = display_name;
        self
    }
}
