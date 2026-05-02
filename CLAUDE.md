# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # Compile
cargo run            # Compile and run (server starts on 127.0.0.1:8080)
cargo check          # Fast type-check without producing binaries
cargo test           # Run tests
cargo clippy         # Lint
```

## Architecture

OauthRS is an OAuth HTTP server built with Axum (async, Tokio runtime).

**Entry point:** `src/main.rs` — prints startup info, calls `start_server()`.

### Structure

```
src/
├── main.rs
├── libs/                    — shared utilities
│   ├── templates.rs         — Tera engine initializer (OnceLock singleton)
│   ├── jwt.rs               — JWT token generation & validation
│   ├── db.rs                — Database connection & queries
│   └── session.rs           — Session management
└── core/
    ├── server.rs            — Axum router + bind (127.0.0.1:8080)
    ├── middleware.rs        — Error handling middleware (404, 405, 401)
    ├── templates/           — Tera templates (.tpl, Jinja2 syntax)
    │   └── errors.tpl
    └── routers/
        ├── home.rs          — GET /
        ├── login.rs         — POST /api/login
        ├── logout.rs        — POST /api/logout
        ├── signup.rs        — POST /api/signup
        └── oauth.rs         — GET /api/oauth
```

### Key conventions

**Routers** return Axum response types — `Json<Value>` for API endpoints, `Html` for pages.

**Templates** use [Tera](https://keats.github.io/tera/) (Jinja2 syntax). Render via `libs::templates::render("file.tpl", &ctx)`. Add new `.tpl` files to `src/core/templates/` — they are picked up automatically.

**Error middleware** (`core/middleware.rs`) intercepts 401, 404, 405 responses and renders `errors.tpl` with `code`, `title`, and `message` context variables.

**`libs/`** is for cross-cutting utilities (JWT, DB, sessions). Wire new libs into `libs/mod.rs` and `main.rs`.
