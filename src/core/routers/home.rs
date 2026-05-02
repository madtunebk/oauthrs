use axum::{extract::State, response::Html};
use tera::Context;

use crate::core::state::AppState;
use crate::libs::templates;

pub async fn handle(State(state): State<AppState>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("app_name", "OauthRS");
    ctx.insert("env",  &std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string()));
    ctx.insert("host", &state.config.host);
    ctx.insert("port", &state.config.port);

    Html(templates::render("index.tpl", &ctx))
}
