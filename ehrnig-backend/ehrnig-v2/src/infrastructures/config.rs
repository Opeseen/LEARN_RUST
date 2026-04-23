use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub environment: String,
    pub rmq_url: String,
    pub email_config: EmailConfig,
    pub redis_url: String,
}

pub struct EmailConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub from: String,
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

            email_config: EmailConfig {
                host: env::var("SMTP_HOST").expect("Email host must be set"),
                port: env::var("SMTP_PORT")
                    .expect("Email port not set")
                    .parse()
                    .unwrap(),
                user: env::var("SMTP_USER").expect("Email user must be set"),
                pass: env::var("SMTP_PASSWORD").expect("Email password not set"),
                from: env::var("FROM_EMAIL").expect("Email from not set"),
            },
            redis_url: env::var("REDIS_URL").expect("Redis url is required"),
        }
    }
}
