use sqlx::PgPool;
use redis::Client;

use crate::libs::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db:     PgPool,
    pub redis:  Client,
    pub config: Config,
}
