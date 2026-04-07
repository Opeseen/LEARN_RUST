use crate::infrastructures::mq::MQPool;
use axum::Router;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub mq: MQPool,
}

pub fn create_route(db_pool: PgPool, mq_pool: MQPool) -> axum::Router {
    let state = AppState {
        db: db_pool,
        mq: mq_pool,
    };

    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/patients", crate::patients::routes())
        .nest("/organizations", crate::organizations::routes())
        .with_state(state)
}
