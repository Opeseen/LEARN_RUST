mod handler;
mod model;
mod repository;

pub use model::*;
pub use repository::*;

use crate::routes::AppState;
use crate::users::handler::create_user;
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(create_user))
}
