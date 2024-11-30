use axum::{
  extract::State,
  routing::get,
  Router,
  Json,
};
use std::sync::Mutex;
use crate::models::cpu_model::CpuInfo;

#[derive(Clone)]
pub struct AppState {
  pub cpu_info: std::sync::Arc<Mutex<CpuInfo>>,
}

async fn get_cpu_info(
  State(state): State<AppState>,
) -> Json<CpuInfo> {
  let mut cpu_info = state.cpu_info.lock().unwrap();
  cpu_info.update();
  Json(cpu_info.clone())
}

pub fn cpu_routes() -> Router<AppState> {
  Router::new()
      .route("/api/cpu", get(get_cpu_info))
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{body::Body, extract::Request};
  use axum::http::StatusCode;
  use tower::ServiceExt;

  #[tokio::test]
  async fn test_get_cpu_info() {
      let app_state = AppState {
          cpu_info: std::sync::Arc::new(Mutex::new(CpuInfo::new())),
      };

      let app = cpu_routes().with_state(app_state);

      let response = app
          .oneshot(Request::builder()
              .uri("/api/cpu")
              .body(Body::empty())
              .unwrap())
          .await
          .unwrap();

      assert_eq!(response.status(), StatusCode::OK);
  }
}