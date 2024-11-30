use std::sync::OnceLock;
use tera::{Context, Error as TeraError, Tera};

static TEMPLATES: OnceLock<Tera> = OnceLock::new();

/// Service for managing HTML templates using Tera
pub struct TemplateService;

impl TemplateService {
    /// Initializes template service with embedded templates
    /// Templates are loaded only once due to OnceLock
    pub fn new() -> Self {
        TEMPLATES.get_or_init(|| {
            let mut tera = Tera::default();
            tera.add_raw_templates(vec![
                ("base.html", include_str!("../../templates/base.html")),
                ("navbar.html", include_str!("../../templates/navbar.html")),
                ("cpu.html", include_str!("../../templates/cpu.html")),
            ])
            .expect("Critical: Failed to initialize templates");
            tera
        });

        TemplateService
    }

    /// Creates context with default values
    pub fn create_default_context() -> Context {
        let mut context = Context::new();
        context.insert("title", "CPU info by A.Š.");
        context
    }

    pub fn render_cpu_page(&self, context: Context) -> Result<String, TeraError> {
        TEMPLATES
            .get()
            .ok_or(TeraError::msg("Templates not initialized"))
            .and_then(|t| t.render("cpu.html", &context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering() {
        let service = TemplateService::new();
        let context = TemplateService::create_default_context();
        let result = service.render_cpu_page(context);
        assert!(result.is_ok());
    }
}
