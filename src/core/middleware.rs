use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tera::Context;

use crate::libs::templates;

pub async fn handle_errors(req: Request<Body>, next: Next) -> Response {
    let response = next.run(req).await;

    match response.status() {
        StatusCode::NOT_FOUND => render_error(404, "Not Found", "The page you are looking for does not exist."),
        StatusCode::METHOD_NOT_ALLOWED => render_error(405, "Method Not Allowed", "This action is not allowed on this endpoint."),
        StatusCode::UNAUTHORIZED => render_error(401, "Unauthorized", "You are not authorized to access this resource."),
        _ => response,
    }
}

fn render_error(code: u16, title: &str, message: &str) -> Response {
    let mut ctx = Context::new();
    ctx.insert("code", &code);
    ctx.insert("title", title);
    ctx.insert("message", message);

    let html = templates::render("errors.tpl", &ctx);

    Response::builder()
        .status(code)
        .header("Content-Type", "text/html")
        .body(Body::from(html))
        .unwrap()
}
