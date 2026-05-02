use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    Form,
};
use serde::Deserialize;
use tera::Context;
use uuid::Uuid;

use crate::core::state::AppState;
use crate::libs::{jwt, models::{AuthResponse, SignupRequest}, password, session, templates};

// GET /api/signup — serve the signup form
pub async fn form() -> Html<String> {
    Html(templates::render("signup.tpl", &Context::new()))
}

// POST /api/signup — JSON (API clients)
pub async fn handle(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user_id = create_user(&state, &body.email, &body.username, &body.password, &body.invite_code)
        .await
        .map_err(|e| e)?;

    let token = jwt::sign(user_id, &state.config.jwt_secret, state.config.jwt_expiry_secs);
    session::set(&state.redis, &format!("session:{}", user_id), &token, state.config.jwt_expiry_secs)
        .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token, token_type: "Bearer".to_string(), expires_in: state.config.jwt_expiry_secs }))
}

// POST /api/signup/form — form submission (browser)
#[derive(Deserialize)]
pub struct SignupForm {
    pub email:       String,
    pub username:    String,
    pub password:    String,
    pub invite_code: String,
}

pub async fn handle_form(
    State(state): State<AppState>,
    Form(body): Form<SignupForm>,
) -> Response {
    match create_user(&state, &body.email, &body.username, &body.password, &body.invite_code).await {
        Ok(user_id) => {
            let token = jwt::sign(user_id, &state.config.jwt_secret, state.config.jwt_expiry_secs);
            let _ = session::set(&state.redis, &format!("session:{}", user_id), &token, state.config.jwt_expiry_secs).await;

            axum::response::Response::builder()
                .status(302)
                .header("Location", "/")
                .header("Set-Cookie", format!("session={}; Path=/; HttpOnly; SameSite=Lax", token))
                .body(axum::body::Body::empty())
                .unwrap()
                .into_response()
        }
        Err(StatusCode::FORBIDDEN)  => render_error("Invalid or expired invite code."),
        Err(StatusCode::CONFLICT)   => render_error("Email or username already taken."),
        Err(_)                      => render_error("Something went wrong. Please try again."),
    }
}

async fn create_user(state: &AppState, email: &str, username: &str, password: &str, invite_code: &str) -> Result<Uuid, StatusCode> {
    let invite_key = format!("invite:{}", invite_code);

    if state.config.invite_required {
        let valid = session::get(&state.redis, &invite_key)
            .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if valid.is_none() {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    let password_hash = password::hash(password);

    let result = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(email)
    .bind(username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok((user_id,)) => {
            // Only consume the invite after a successful insert
            let _ = session::del(&state.redis, &invite_key).await;
            Ok(user_id)
        }
        Err(sqlx::Error::Database(e)) if e.code().as_deref() == Some("23505") => {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn render_error(msg: &str) -> Response {
    let mut ctx = Context::new();
    ctx.insert("error", msg);
    Html(templates::render("signup.tpl", &ctx)).into_response()
}
