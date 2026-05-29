# OauthRS

A lightweight authentication microservice built with Rust and [Axum](https://github.com/tokio-rs/axum). Handles user registration, login, session management, Google OAuth, and exposes a `/auth` endpoint compatible with **nginx `auth_request`**.

## Features

- Email/password login and signup (Argon2 password hashing)
- Google OAuth 2.0 login
- JWT session tokens stored in Redis
- Invite-only registration mode
- `GET /auth` — nginx `auth_request` subrequest endpoint (validates JWT from `Authorization` header or `session` cookie)
- OAuth 2.0 token issuance and revoke endpoints
- Tera templates (Jinja2 syntax) for login/signup pages
- Auto-runs SQLx migrations on startup

## Stack

| Layer | Tech |
|---|---|
| HTTP | Axum 0.8 + Tokio |
| Database | PostgreSQL 16 via SQLx |
| Sessions | Redis |
| Auth | JWT (jsonwebtoken) + Argon2 |
| Templates | Tera |

## Getting started

**Requirements:** Rust (stable), PostgreSQL 16, Redis

```bash
git clone https://github.com/madtunebk/OauthRS.git
cd OauthRS
cp .env.example .env   # fill in your values
cargo run
```

Server starts on `http://127.0.0.1:8080` by default.

## Environment variables

Copy `.env.example` to `.env` and set:

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | yes | — | PostgreSQL connection string |
| `REDIS_URL` | yes | — | Redis connection string |
| `JWT_SECRET` | yes | — | Secret key for signing JWTs |
| `JWT_EXPIRY_SECS` | no | `3600` | Token lifetime in seconds |
| `HOST` | no | `127.0.0.1` | Bind address |
| `PORT` | no | `8080` | Bind port |
| `ADMIN_SECRET` | yes | — | Secret for admin operations |
| `INVITE_REQUIRED` | no | `true` | Require invite code to register |
| `INVITE_TTL_SECS` | no | `86400` | Invite link expiry |
| `GOOGLE_CLIENT_ID` | no | — | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | no | — | Google OAuth client secret |
| `GOOGLE_REDIRECT_URI` | no | — | OAuth callback URL |

## API routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/` | Home page |
| `GET` | `/auth` | nginx `auth_request` — returns 200 or 401 |
| `GET/POST` | `/login` | Login page / submit |
| `GET/POST` | `/signup` | Signup page / submit |
| `POST` | `/logout` | Invalidate session |
| `GET` | `/auth/google` | Start Google OAuth flow |
| `GET` | `/auth/google/callback` | Google OAuth callback |
| `POST` | `/oauth/token` | Issue OAuth token |
| `POST` | `/oauth/revoke` | Revoke OAuth token |
| `GET` | `/oauth/authorize` | OAuth authorization |
| `POST` | `/api/invite` | Generate invite link |

## nginx integration

```nginx
location /protected {
    auth_request /auth;
    auth_request_set $auth_status $upstream_status;
    error_page 401 = /login;
    # ... your proxy config
}

location = /auth {
    internal;
    proxy_pass http://127.0.0.1:8080/auth;
    proxy_pass_request_body off;
    proxy_set_header Content-Length "";
}
```

The `/auth` endpoint reads the JWT from:
1. `Authorization: Bearer <token>` header
2. `session` cookie

Returns `200 OK` for a valid, active session — `401 Unauthorized` otherwise.

## Development

```bash
cargo check      # fast type-check
cargo clippy     # lint
cargo test       # run tests
cargo build      # compile
cargo run        # compile + start server
```

## License

MIT
