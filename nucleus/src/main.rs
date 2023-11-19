use dotenvy::dotenv;
use mongodb::{Client, error::Error, bson::doc};
use std::env;
use axum::{
    Router, 
    routing::get, 
    body::Body, 
    Extension, 
    middleware,
};

use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

mod codex;
mod error;
mod model;
mod app;


#[derive(Clone)]
struct AppState {
    db: mongodb::Client,
}


#[tokio::main]
async fn main() -> error::Result<()>{
    dotenv().ok();

    let db_client = db_connection().await?;

    let app = routes(db_client).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> {:<12} - {addr}\n", "LISTENING");
    axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn routes(db: Client) -> Router<(), Body> {
    let codex_routes = codex::routes::create_route().await;

    let state = AppState {
        db,
    };

    Router::new()
        .route("/", get(|| async {"this is the root"}))
        .nest("/codex", codex_routes)
        // .layer(middleware::map_response(mw_reponse_map))
		// .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
}



async fn db_connection() -> Result<Client, Error> {

    let uri = env::var("MONGO_DB_URI").unwrap();
    

    let client = Client::with_uri_str(uri).await?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        
        .await?;

    Ok(client)
}