use crate::codex::handlers;

use axum::{
    Router, 
    routing::{
        get, 
        post, 
        delete, 
        patch
    }, 
    body::Body, 
    http::Method
};
use tower_http::cors::{CorsLayer, Any};


pub async fn create_route() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(Any);


    // let db_uri = env::var("DATABASE_URI").unwrap();
    // let conn = mongodb::Client::


    Router::new()
        .route("/entries", post(handlers::create_entry))
        .route("/entries", get(handlers::load_entries))
        .route("/entries/:id", get(handlers::load_one_entry))
        .route("/entries/:id", patch(handlers::update_entry))
        .route("/entries/:id", delete(handlers::delete_entry))
        // .route("/tags", post(handlers::create_tag))
        // .route("/tags", get(handlers::load_tags))
        // .route("/tags/:id", get(handlers::load_one_tag))
        // .route("/tags/:id", patch(handlers::update_tag))
        // .route("/tags/:id", delete(handlers::delete_tag))
        .layer(cors)
}