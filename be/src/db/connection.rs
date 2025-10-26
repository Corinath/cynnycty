use arcadedb_rs::{ArcadeDB, Auth};
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseConnection {
    pub client: Arc<ArcadeDB>,
    pub database_name: String,
}

impl DatabaseConnection {
    pub async fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        database_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!("http://{}:{}", host, port);

        tracing::info!("Connecting to ArcadeDB at {}", url);

        let client = ArcadeDB::builder()
            .auth(Auth::basic(username, password))
            .build(&url)
            .await?;

        Ok(Self {
            client: Arc::new(client),
            database_name: database_name.to_string(),
        })
    }

    pub async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Simple health check by querying the database
        let db = self.client.db(&self.database_name);

        // Try a simple query to verify connection
        let _result = db
            .query("SELECT 1 as health")
            .send::<serde_json::Value>()
            .await?;

        Ok(true)
    }

    pub fn get_db(&self) -> arcadedb_rs::Database {
        self.client.db(&self.database_name)
    }
}

pub async fn init_database() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let host = std::env::var("ARCADE_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("ARCADE_DB_PORT")
        .unwrap_or_else(|_| "2480".to_string())
        .parse::<u16>()
        .unwrap_or(2480);
    let username = std::env::var("ARCADE_DB_USER").unwrap_or_else(|_| "root".to_string());
    let password = std::env::var("ARCADE_DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let database_name = std::env::var("ARCADE_DB_NAME").unwrap_or_else(|_| "cynnycty".to_string());

    DatabaseConnection::new(&host, port, &username, &password, &database_name).await
}
