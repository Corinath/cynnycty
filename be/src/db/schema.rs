use crate::db::connection::DatabaseConnection;

/// Initialize the database schema
/// This creates all necessary types, properties, and indexes
pub async fn init_schema(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing database schema...");

    // Define schema statements in order
    let statements = vec![
        // 1. Create the Profile document type
        "CREATE DOCUMENT TYPE Profile",

        // 2. Create properties
        "CREATE PROPERTY Profile.userId STRING",
        "CREATE PROPERTY Profile.clerkId STRING",
        "CREATE PROPERTY Profile.displayName STRING",
        "CREATE PROPERTY Profile.aboutMe STRING",
        "CREATE PROPERTY Profile.avatarUrl STRING",
        "CREATE PROPERTY Profile.createdAt DATETIME",
        "CREATE PROPERTY Profile.updatedAt DATETIME",

        // 3. Create indexes
        "CREATE INDEX Profile_userId_idx ON Profile (userId) UNIQUE",
        "CREATE INDEX Profile_clerkId_idx ON Profile (clerkId) UNIQUE",
    ];

    tracing::info!("Executing {} schema statements...", statements.len());

    // Get environment variables for database connection
    let host = std::env::var("ARCADE_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("ARCADE_DB_PORT")
        .unwrap_or_else(|_| "2480".to_string());
    let username = std::env::var("ARCADE_DB_USER").unwrap_or_else(|_| "root".to_string());
    let password = std::env::var("ARCADE_DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let database_name = std::env::var("ARCADE_DB_NAME").unwrap_or_else(|_| "cynnycty".to_string());

    let url = format!("http://{}:{}/api/v1/command/{}", host, port, database_name);
    let client = reqwest::Client::new();

    for (idx, statement) in statements.iter().enumerate() {
        tracing::debug!("Executing statement {}: {}", idx + 1, statement);

        let payload = serde_json::json!({
            "language": "sql",
            "command": statement
        });

        match client
            .post(&url)
            .basic_auth(&username, Some(&password))
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    tracing::info!("✓ Statement {} executed successfully", idx + 1);
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

                    // Check if error is about already existing type/property/index
                    if error_text.contains("already exists")
                        || error_text.contains("already defined")
                        || error_text.contains("Type 'Profile' already exists") {
                        tracing::info!("○ Statement {} skipped (already exists)", idx + 1);
                    } else {
                        tracing::error!("Failed to execute statement {}: Status {}", idx + 1, status);
                        tracing::error!("Error: {}", error_text);
                        tracing::error!("Statement was: {}", statement);
                        return Err(format!("Schema initialization failed at statement {}", idx + 1).into());
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to execute statement {}: {}", idx + 1, e);
                tracing::error!("Statement was: {}", statement);
                return Err(e.into());
            }
        }
    }

    tracing::info!("Database schema initialization complete");
    Ok(())
}

/// Check if the schema has been initialized by checking for the Profile type
pub async fn is_schema_initialized(
    db: &DatabaseConnection,
) -> Result<bool, Box<dyn std::error::Error>> {
    let database = db.get_db();

    match database
        .query("SELECT FROM schema:types WHERE name = 'Profile'")
        .send::<serde_json::Value>()
        .await
    {
        Ok(result) => {
            tracing::debug!("Schema check result: {:?}", result);
            Ok(!result.is_empty())
        }
        Err(e) => {
            tracing::warn!("Error checking schema: {}", e);
            Ok(false)
        }
    }
}
