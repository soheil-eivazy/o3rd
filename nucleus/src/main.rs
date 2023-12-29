mod error;
mod config;
mod sketch;


use axum::{Router, routing::get, Extension};

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use error::Result;
use std::sync::Arc;


#[tokio::main]
async fn main() -> error::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();


    

    let app_state = AppState::new().await?; 
    let routes = Router::new()
        .route("/", get(|| async {"this is the root"}))
        .nest("/sketch", sketch::routes())
        .layer(Extension(Arc::new(app_state)));
        

    // .layer(middleware::map_response(mw_reponse_map))
    // .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
    // .layer(CookieManagerLayer::new());


    let port = "0.0.0.0:3003";
    let listener = TcpListener::bind(port).await.unwrap();

    info!("LISTENING - {listener:?}\n");
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();


    Ok(())
}


pub struct AppState {
    pub db: mongodb::Database,
}

impl AppState {
    async fn new() -> Result<AppState> {
        let conf = config::load_config();

        Ok(AppState {
            db: mongodb::Client::with_uri_str(&conf.MONGO_DB_URI).await?.database("o3rd"),
        })
    }
}