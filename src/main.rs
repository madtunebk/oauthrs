mod core;
mod libs;

use core::server::start_server;
use libs::config::Config;
use libs::{db, session};

pub const APP_NAME: &str = "OauthRS";
pub const APP_ENV: &str = "dev";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    tracing::info!("Starting {} in {} mode", APP_NAME, APP_ENV);

    let config     = Config::load();
    let db_pool    = db::connect(&config.database_url).await;
    let redis      = session::connect(&config.redis_url);

    db::run_migrations(&db_pool).await;
    tracing::info!("Migrations applied");

    start_server(config, db_pool, redis).await;
}
