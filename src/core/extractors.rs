use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::core::state::AppState;
use crate::libs::jwt;

pub struct AuthSession {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthSession {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // 1. try Authorization: Bearer <token> header
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        // 2. fallback to session cookie
        let token = token.or_else(|| {
            parts
                .headers
                .get(header::COOKIE)
                .and_then(|v| v.to_str().ok())
                .and_then(|cookies| {
                    cookies
                        .split(';')
                        .find(|c| c.trim().starts_with("session="))
                        .map(|c| c.trim().trim_start_matches("session=").to_string())
                })
        });

        let token = token.ok_or(StatusCode::UNAUTHORIZED)?;

        // 3. verify JWT
        let claims = jwt::verify(&token, &state.config.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthSession { user_id })
    }
}
