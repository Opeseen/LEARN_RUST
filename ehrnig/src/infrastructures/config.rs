use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub environment: String,
    pub rmq_url: String,
}

impl Config {
    pub fn load_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://dev_user:adminuser@localhost:5432/ehrdb".to_string()
            }),

            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),

            environment: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            rmq_url: env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
        }
    }
}
