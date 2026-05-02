use axum::{
    extract::State,
    http::{header, HeaderMap},
    response::{IntoResponse, Redirect, Response},
};

use crate::core::state::AppState;
use crate::libs::jwt;

// GET /oauth/authorize
// Checks if the request carries a valid session (header or cookie).
// Valid   → returns authorized user info (or consent page later)
// Invalid → redirects to /api/login
pub async fn handle(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let token = extract_token(&headers);

    match token.and_then(|t| jwt::verify(&t, &state.config.jwt_secret).ok()) {
        Some(_) => Redirect::to("/").into_response(),

        None => Redirect::to("/login").into_response(),
    }
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    // 1. Authorization: Bearer <token>
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        // 2. session cookie
        .or_else(|| {
            headers
                .get(header::COOKIE)
                .and_then(|v| v.to_str().ok())
                .and_then(|cookies| {
                    cookies
                        .split(';')
                        .find(|c| c.trim().starts_with("session="))
                        .map(|c| c.trim().trim_start_matches("session=").to_string())
                })
        })
}
