pub mod handler;
pub mod model;
pub mod repository;

pub use model::*;

use crate::patients::handler::{create_patient, get_all_patient, get_patient};
use crate::routes::AppState;
use axum::{Router, routing::get, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_patient))
        .route("/{:id}", get(get_patient))
        .route("/", post(create_patient))
}
