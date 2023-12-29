mod controller;
mod model;

use axum::{Router, routing::{get, put, post, delete}};




pub fn routes() -> Router {
    Router::new()
        .route("/:id", get(controller::load_one))
        .route("", get(controller::load_many))
        .route("", post(controller::create))
        .route("/:id", put(controller::update))
        .route("/:id", delete(controller::delete))
}