use std::env;
use dotenv::dotenv;

pub struct AppConfig {
    pub host: String,
    pub port: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        AppConfig { host, port }
    }
}