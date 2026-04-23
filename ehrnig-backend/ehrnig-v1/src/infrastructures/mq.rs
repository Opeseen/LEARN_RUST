use deadpool_lapin::{Config, Pool, Runtime};

pub type MQPool = Pool;

pub fn create_mq_pool(url: String) -> MQPool {
    let mut cfg = Config::default();
    cfg.url = Some(url);

    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create RabbitMQ pool")
}
