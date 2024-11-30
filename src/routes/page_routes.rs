use axum::{
  routing::get,
  Router,
  response::Html,
};
use crate::routes::cpu_routes::AppState;
use crate::templates::template_service::TemplateService;

async fn render_cpu_page() -> Html<String> {
  let service = TemplateService::new();
  let context = TemplateService::create_default_context();
  Html(service.render_cpu_page(context).unwrap_or_else(|_| "Error".to_string()))
}

pub fn page_routes() -> Router<AppState> {
  Router::new()
      .route("/cpu", get(render_cpu_page))
      .route("/", get(render_cpu_page))
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::{body::Body, extract::Request};
  use axum::http::StatusCode;
  use tower::ServiceExt;
  use crate::models::cpu_model::CpuInfo;
  use std::sync::Mutex;

  #[tokio::test]
  async fn test_render_cpu_page() {
      let app_state = AppState {
          cpu_info: std::sync::Arc::new(Mutex::new(CpuInfo::new())),
      };

      let app = page_routes().with_state(app_state);

      let response = app
          .oneshot(Request::builder()
              .uri("/cpu")
              .body(Body::empty())
              .unwrap())
          .await
          .unwrap();

      assert_eq!(response.status(), StatusCode::OK);
  }
}