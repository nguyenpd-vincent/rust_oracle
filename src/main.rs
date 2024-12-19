use actix_web::{App, HttpServer, middleware::Logger};
use crate::routes::post_routes::init_routes as init_post_routes;
use crate::config::app_config::AppConfig;
use crate::utils::logging::init_logging;

#[cfg(feature = "database_oracle")]
use crate::config::oracle_database::OracleDatabase;

mod config;
mod models;
mod repositories;
mod services;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    init_logging();
    // env_logger::init();
    let config = AppConfig::new();
    // Test the connection
    #[cfg(feature = "database_oracle")]
    match OracleDatabase::get_connection() {
        Ok(_conn) => println!("Connected to Oracle database."),
        Err(err) => eprintln!("Failed to connect to the Oracle database: {}", err),
    }

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(init_post_routes)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}