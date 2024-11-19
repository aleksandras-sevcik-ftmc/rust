// src/routes/ascii.rs
/**
* ASCII art route handlers
* Responsibilities:
* - Handle ASCII art endpoints
* - Process image upload
* - Manage ASCII conversion requests
* - Return ASCII art responses
*/

use axum::{
  response::{Html, IntoResponse, Response},
  routing::{get, post},
  extract::{Multipart, Query},
  Router,
};

use crate::services::ascii_service::AsciiService;
use crate::templates::template_service::TemplateService;
use serde::Deserialize;

#[derive(Deserialize)]
struct DownloadParams {
    ascii: String,
}

pub fn ascii_routes() -> Router {
  Router::new()
      .route("/", get(show_form))
      .route("/convert", post(convert_image))
      .route("/download", get(download_ascii))
}

async fn show_form() -> Html<String> {
  let template_service = TemplateService::new();
  let context = TemplateService::create_default_context();
  Html(template_service.render_ascii_page(context))
}

async fn convert_image(mut multipart: Multipart) -> Html<String> {
  let service = AsciiService::new();
  let template_service = TemplateService::new();
  let mut context = TemplateService::create_default_context();

  if let Some(field) = multipart.next_field().await.unwrap() {
      if let Ok(data) = field.bytes().await {
          if let Ok(img) = image::load_from_memory(&data) {
              match service.image_to_ascii(&img) {
                  Ok(ascii_art) => {
                      // Use AsciiArt model properly
                      context.insert("ascii_result", &ascii_art);
                      context.insert("width", &ascii_art.width);
                      context.insert("height", &ascii_art.height);
                      context.insert("success", &true);
                  },
                  Err(e) => context.insert("error", &e.to_string()),
              }
          }
      }
  }

  Html(template_service.render_ascii_page(context))
}

// Modified download handler
async fn download_ascii(
  Query(params): Query<DownloadParams>  // Extract from query params
) -> impl IntoResponse {
  Response::builder()
      .header("content-type", "text/plain")
      .header("content-disposition", "attachment; filename=\"ascii-art.txt\"")
      .body(params.ascii)
      .unwrap()
}