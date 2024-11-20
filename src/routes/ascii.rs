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
    extract::{Multipart, Path, State},
    response::{Html, Response},
    routing::{get, post},
    Router,
    body::Body,
};

use crate::services::ascii_service::AsciiService;
use crate::templates::template_service::TemplateService;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

type SharedState = Arc<AppState>;

pub struct AppState {
    ascii_storage: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ascii_storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn ascii_routes() -> Router<SharedState> {
  let state = Arc::new(AppState::new());
  
  Router::new()
      .route("/", get(show_form))
      .route("/convert", post(convert_image))
      .route("/download/:id", get(download_ascii))
      .with_state(state)
}


#[axum::debug_handler]
async fn show_form() -> Html<String> {
    let template_service = TemplateService::new();
    let context = TemplateService::create_default_context();
    Html(template_service.render_ascii_page(context))
}


#[axum::debug_handler]
async fn convert_image(
  state: State<SharedState>,  // Changed this line
  mut multipart: Multipart,
) -> Html<String> {
  let service = AsciiService::new();
  let template_service = TemplateService::new();
  let mut context = TemplateService::create_default_context();

  if let Some(field) = multipart.next_field().await.unwrap() {
      if let Ok(data) = field.bytes().await {
          if let Ok(img) = image::load_from_memory(&data) {
              match service.image_to_ascii(&img) {
                  Ok(ascii_art) => {
                      let id = Uuid::new_v4().to_string();
                      state
                          .ascii_storage  // Remove .0 accessor
                          .lock()
                          .await
                          .insert(id.clone(), ascii_art.content.clone());

                      context.insert("download_id", &id);
                      context.insert("ascii_result", &ascii_art);
                      context.insert("width", &ascii_art.width);
                      context.insert("height", &ascii_art.height);
                      context.insert("success", &true);
                  }
                  Err(e) => context.insert("error", &e.to_string()),
              }
          }
      }
  }

  Html(template_service.render_ascii_page(context))
}


#[axum::debug_handler]
async fn download_ascii(
  state: State<SharedState>,  // Changed this line
  Path(id): Path<String>,
) -> Response<Body> {
  if let Some(ascii_art) = state
      .ascii_storage  // Remove .0 accessor
      .lock()
      .await
      .get(&id) 
  {
      Response::builder()
          .header("content-type", "text/plain")
          .header(
              "content-disposition",
              "attachment; filename=\"ascii-art.txt\"",
          )
          .body(Body::from(ascii_art.clone()))  // Changed to Body::from
          .unwrap()
  } else {
      Response::builder()
          .status(404)
          .body(Body::from("ASCII art not found"))  // Changed to Body::from
          .unwrap()
  }
}