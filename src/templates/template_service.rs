// src/templates/template_service.rs
/**
* Template rendering service
* Responsibilities:
* - Initialize Tera
* - Manage template context
* - Render HTML templates
* - Handle template errors
*/
use tera::{Tera, Context};
use std::sync::OnceLock;

static TEMPLATES: OnceLock<Tera> = OnceLock::new();

pub struct TemplateService;

impl TemplateService {
   pub fn new() -> Self {
       TEMPLATES.get_or_init(|| {
           let mut tera = Tera::default();
           tera.add_raw_templates(vec![
               ("base.html", include_str!("../../templates/base.html")),
               ("ascii.html", include_str!("../../templates/ascii.html")),
           ]).expect("Failed to initialize templates");
           tera
       });
       
       TemplateService
   }

   pub fn create_default_context() -> Context {
       let mut context = Context::new();
       context.insert("title", "ASCII Art Generator");
       context
   }

   pub fn render_ascii_page(&self, context: Context) -> String {
       TEMPLATES
           .get()
           .expect("Templates not initialized")
           .render("ascii.html", &context)
           .unwrap_or_else(|e| format!("Error rendering template: {}", e))
   }
}