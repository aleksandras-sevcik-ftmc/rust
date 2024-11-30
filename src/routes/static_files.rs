use axum::{
    Router,
    routing::get_service,
};
use tower_http::services::ServeDir;
use crate::routes::cpu_routes::AppState;

pub fn static_routes() -> Router<AppState> {
    Router::new()
        .nest_service(
            "/static",
            get_service(ServeDir::new("static"))
                .handle_error(|error| async move {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        )
}