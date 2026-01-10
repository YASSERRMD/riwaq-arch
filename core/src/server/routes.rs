//! HTTP router configuration.

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use super::handlers;
use super::state::AppState;

/// Create the main router with all routes.
pub fn create_router(state: AppState) -> Router {
    // CORS configuration for VS Code extension
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Health check
        .route("/health", get(handlers::health))
        
        // Analysis endpoints
        .route("/analyze", post(handlers::analyze))
        .route("/projects", get(handlers::list_projects))
        
        // Documentation endpoints
        .route("/docgen", post(handlers::generate_docs))
        
        // Q&A endpoints
        .route("/ask", post(handlers::ask_question))
        
        // Diagram endpoints
        .route("/architecture/diagram", get(handlers::get_architecture_diagram))
        .route("/dependencies/diagram", get(handlers::get_dependency_diagram))
        
        // Apply middleware
        .layer(cors)
        .with_state(state)
}

/// Run the HTTP server.
pub async fn run_server(host: &str, port: u16, state: AppState) -> std::io::Result<()> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("Starting Riwaq server on {}", addr);
    
    let router = create_router(state);
    
    axum::serve(listener, router).await?;
    
    Ok(())
}
