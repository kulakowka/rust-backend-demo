use std::env;
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub rust_log: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid u16");
        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Config {
            database_url,
            server_host,
            server_port,
            rust_log,
        }
    }
}
