use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_secs: u64,
    pub host:         String,
    pub port:         u16,
    pub admin_secret:    String,
    pub invite_ttl_secs: u64,
    pub invite_required: bool,
    pub google_client_id:     String,
    pub google_client_secret: String,
    pub google_redirect_uri:  String,
}

impl Config {
    pub fn load() -> Self {
        Config {
            database_url:    env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url:       env::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_secret:      env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiry_secs: env::var("JWT_EXPIRY_SECS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .expect("JWT_EXPIRY_SECS must be a number"),
            host:         env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port:         env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            admin_secret:    env::var("ADMIN_SECRET").expect("ADMIN_SECRET must be set"),
            invite_ttl_secs: env::var("INVITE_TTL_SECS")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .expect("INVITE_TTL_SECS must be a number"),
            invite_required: env::var("INVITE_REQUIRED")
                .unwrap_or_else(|_| "true".to_string())
                .trim()
                .to_lowercase()
                != "false",
            google_client_id:     env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            google_redirect_uri:  env::var("GOOGLE_REDIRECT_URI").unwrap_or_default(),
        }
    }
}
