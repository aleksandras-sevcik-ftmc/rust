// src/routes/static_files.rs
/**
* Static file serving
* Responsibilities:
* - Serve static assets (JS, CSS)
* - Configure caching
* - Handle static file routes
*/

// Import required modules from axum and tower-http
use axum::Router;  // For creating web routes
use tower_http::services::ServeDir; // For serving static files

// Function that creates and returns a Router for static file serving
pub fn static_routes() -> Router {
   Router::new()                   // Create new router instance
       .nest_service(              // Add nested service route
           "/static",              // URL path for static files
           ServeDir::new("static") // Serve files from "static" directory
       )
}