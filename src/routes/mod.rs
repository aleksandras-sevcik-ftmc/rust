// src/routes/mod.rs
/**
* Routes module configuration
* Responsibilities:
* - Aggregate all route modules
* - Create and configure main Router
* - Manage route hierarchies
*/
mod ascii;           // ASCII art route handling module
mod static_files;    // Static file serving module

// Re-export all public items from ascii module
pub use ascii::*;
// Import Router from axum framework    
use axum::Router;

// Public function to create and configure main application router
pub fn create_router() -> Router {
   Router::new()                           // Create new router
       .merge(ascii_routes())             // Add ASCII routes
       .merge(static_files::static_routes()) // Add static file routes
}