mod infrastructures;
mod jobs;
mod labs;
mod organizations;
mod patients;
mod routes;
mod users;
mod utility;

use infrastructures::{config::Config, db, mq};
use jobs::work;
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
    let pool = db::create_pool(&config)
        .await
        .expect("Failed to connect to the database!.");

    // Initialize RabbitMQ Pool
    let mq_pool = mq::create_mq_pool(config.rmq_url);

    // Start RabbiMQ Background workers and we close the
    // queue so the background thread has its own handle
    let worker_pool = mq_pool.clone();
    tokio::spawn(async move {
        work::start_workers(worker_pool).await;
    });

    // setup router
    let app = routes::create_route(pool, mq_pool);

    // start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!(
        " EHR Server starting on {}: Port {}",
        config.environment,
        addr.port()
    );

    // listen and serve
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
