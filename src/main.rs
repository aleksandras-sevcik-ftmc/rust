mod models;
mod routes;
mod services;
mod templates;

use routes::{create_router, cpu_routes::AppState};
use services::cpu_services::CpuService;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cpu_service = CpuService::new();
   
    let app_state = AppState {
        cpu_info: cpu_service.get_state(),
    };

    let app = create_router(app_state);
    Ok(app.into())
}