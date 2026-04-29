use axum::Router;
use sqlx::PgPool;
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tower_sessions_redis_store::fred::clients::RedisClient;
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisClient,
}

pub fn create_route(db_pool: PgPool, redis_client: RedisClient) -> axum::Router {
    // configure the session store
    let session_store = RedisStore::new(redis_client.clone());

    // build the session layer
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("ehr_session")
        .with_secure(false) // for development use case
        .with_expiry(Expiry::OnInactivity(Duration::hours(8)));

    let state = AppState {
        db: db_pool,
        redis: redis_client,
    };

    // create auth route and apply session layer only here for the auth server use case
    let auth_routes = crate::auth::routes().route_layer(session_layer);

    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/auth", auth_routes)
        .nest("/patients", crate::patients::routes())
        .nest("/organizations", crate::organizations::routes())
        .nest("/users", crate::users::routes())
        .with_state(state)
}
