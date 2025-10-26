mod routes;
mod db;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use routes::health::health_check;
use routes::database::database_health_check;
use db::connection::init_database;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cynnycty_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database connection
    let db = match init_database().await {
        Ok(db) => {
            tracing::info!("Database connection established successfully");
            db
        }
        Err(e) => {
            tracing::warn!("Failed to connect to database: {}. Server will start without database connection.", e);
            tracing::info!("Database health check endpoint will be unavailable");
            // For now, we'll panic. In production, you might want to continue without DB
            panic!("Database connection required for startup");
        }
    };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/db/health", get(database_health_check))
        .with_state(db)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
