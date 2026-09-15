//! One-line Axum server builder.

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::health;

/// Build and run an Axum server with common middleware and health routes.
///
/// # Example
///
/// ```ignore
/// use axum_server::server;
///
/// let app = axum::Router::new()
///     .route("/api/hello", get(|| async { "world" }));
///
/// server::serve(app, "0.0.0.0:3000").await.unwrap();
/// ```
pub async fn serve(app: Router, addr: &str) -> Result<(), ServerError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| ServerError::Bind(e.to_string()))?;

    info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_kit::shutdown_signal())
        .await
        .map_err(|e| ServerError::Serve(e.to_string()))?;

    info!("Server shut down gracefully");
    Ok(())
}

/// Build a router with health routes and optional middleware.
pub fn with_health(app: Router, health_registry: healthkit::HealthRegistry) -> Router {
    app.merge(health::health_routes(health_registry))
}

/// Errors that can occur during server startup.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    /// Failed to bind to address.
    #[error("failed to bind to address: {0}")]
    Bind(String),

    /// Server error during operation.
    #[error("server error: {0}")]
    Serve(String),
}
