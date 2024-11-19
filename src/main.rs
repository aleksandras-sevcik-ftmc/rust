// src/main.rs
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
mod models;

use routes::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = "127.0.0.1:8000";

    println!("Server running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
