mod handler;
mod model;
mod repository;

pub use model::*;

use crate::organizations::handler::{create_organization, get_all_organization, get_organization};
use crate::routes::AppState;
use axum::{Router, routing::get, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_organization))
        .route("/{:id}", get(get_organization))
        .route("/", post(create_organization))
}
