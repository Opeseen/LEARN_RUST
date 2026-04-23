use tower_sessions_redis_store::fred::clients::RedisClient;
use tower_sessions_redis_store::fred::prelude::*;

pub async fn create_redis_client(redis_url: String) -> RedisClient {
    // create the redis config from the url
    let config = RedisConfig::from_url(&redis_url).expect("Invalid Redis Url");
    // initialize the redis client
    let client = RedisClient::new(config, None, None, None);
    // start the connection task
    let _ = client.connect();
    // wait for the connection to be ready
    client
        .wait_for_connect()
        .await
        .expect("Failed to connect to redis server");

    client
}
