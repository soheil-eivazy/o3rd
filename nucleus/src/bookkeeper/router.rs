use axum::{Router, routing::{get, put, post, delete}};

use super::account;

pub fn routes() -> Router {
    Router::new()
        .route("account/:id", get(account::load_one))
        .route("account", get(account::load_many))
        .route("account", post(account::create))
        .route("account/:id", put(account::update))
        .route("account/:id", delete(account::delete))
}