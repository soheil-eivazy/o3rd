use mongodb::{Client, error::Error, bson::doc};
use std::env;
use axum::{Router, routing::{get}, body::Body, Extension};

use crate::codex;

pub async fn routes(db: Client) -> Router<(), Body> {
    let codex_routes = codex::routes::create_route().await;


    Router::new()
        .route("/", get(|| async {"this is the root"}))
        .nest("/codex", codex_routes)
        .layer(Extension(db))
}



pub async fn db_connection() -> Result<Client, Error> {

    let uri = env::var("MONGO_DB_URI").unwrap();

    let client = Client::with_uri_str(uri).await?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client)
}