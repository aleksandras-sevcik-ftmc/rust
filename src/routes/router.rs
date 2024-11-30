use axum::Router;
use crate::routes::{
    cpu_routes::AppState,
    cpu_routes,
    static_files,
};
use crate::routes::page_routes;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .merge(cpu_routes::cpu_routes())
        .merge(page_routes::page_routes())
        .merge(static_files::static_routes())
        .with_state(app_state)
}