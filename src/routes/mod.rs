/**
* Routes module configuration
* Responsibilities:
* - Aggregate all route modules
* - Create and configure main Router
* - Manage route hierarchies
*/
mod ascii; // ASCII art route handling module
mod static_files; // Static file serving module

// Re-export all public items from ascii module
pub use ascii::*;
use axum::Router;
use std::sync::Arc;

// Public function to create and configure main application router
pub fn create_router() -> Router {
    // Create shared state
    let state = Arc::new(AppState::new());

    // Create base router with ASCII routes and state
    let app = Router::new()
        .merge(ascii_routes())
        .merge(static_files::static_routes());

    // Apply state to the entire router
    app.with_state(state)
}
