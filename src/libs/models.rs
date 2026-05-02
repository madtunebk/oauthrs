use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub username:      String,
    pub password_hash: String,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email:       String,
    pub username:    String,
    pub password:    String,
    pub invite_code: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login:    String,  // accepts email or username
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token:      String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub grant_type:    String,
    pub login:         Option<String>,  // accepts email or username
    pub password:      Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type:   String,
    pub expires_in:   u64,
}

#[derive(Deserialize)]
pub struct RevokeRequest {
    pub token: String,
}
