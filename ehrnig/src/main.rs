mod events;
mod infrastructures;
mod jobs;
mod labs;
mod organizations;
mod patients;
mod routes;
mod users;
mod utility;

use infrastructures::{config::Config, db, email::EmailService, mq};
use jobs::{relay, workers};
use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("ehrnig=info,lapin=warn,deadpool_lapin=warn"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    // load env and connect db
    let config = Config::load_env();
    let db_pool = db::create_pool(config.database_url)
        .await
        .expect("Failed to connect to the database!.");

    // initialize the emil service
    let email_service = EmailService::new(config.email_config);

    // Initialize RabbitMQ Pool
    let mq_pool = mq::create_mq_pool(config.rmq_url);

    // creates clones for background task
    let relay_db_pool = db_pool.clone();
    let relay_mq_pool = mq_pool.clone();
    let worker_pool = mq_pool.clone();
    let email_service = email_service.clone();

    // Start the Workers (DB -> MQ) in Background
    tokio::spawn(async move {
        relay::start_relay_worker(relay_db_pool, relay_mq_pool).await;
    });

    tokio::spawn(async move {
        workers::start_workers(worker_pool, email_service).await;
    });

    // setup router (pass mq_pool to state for future use case)
    let app = routes::create_route(db_pool, mq_pool);

    // start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!(
        " EHR Server starting on {}: host: {}:{}",
        config.environment,
        addr.ip(),
        addr.port()
    );

    // listen and serve
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
