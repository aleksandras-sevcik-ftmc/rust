/**
* Static file serving
* Responsibilities:
* - Serve static assets (JS, CSS)
* - Configure caching
* - Handle static file routes
*/

use axum::Router;
use tower_http::services::ServeDir;
use std::sync::Arc;
use crate::routes::ascii::AppState;  // Import AppState type

// Updated to use same state type as ASCII routes
pub fn static_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest_service(
            "/static",
            ServeDir::new("static")
        )
}