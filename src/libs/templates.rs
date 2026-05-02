use std::sync::OnceLock;
use tera::Tera;

static TERA: OnceLock<Tera> = OnceLock::new();

pub fn engine() -> &'static Tera {
    TERA.get_or_init(|| {
        let dir = std::env::var("TEMPLATES_DIR")
            .unwrap_or_else(|_| "src/core/templates".to_string());
        let pattern = format!("{}/**/*.tpl", dir);
        Tera::new(&pattern).expect("Failed to load templates")
    })
}

pub fn render(template: &str, ctx: &tera::Context) -> String {
    engine().render(template, ctx).expect("Failed to render template")
}
