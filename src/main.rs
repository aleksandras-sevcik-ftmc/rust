// src/main.rs
mod models;
/**
* Main application entry point
* Responsibilities:
* - Initialize application components
* - Configure and start web server
* - Set up routing
*/
mod routes;
mod services;
mod templates;

use routes::create_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = create_router();
    Ok(app.into())
}
