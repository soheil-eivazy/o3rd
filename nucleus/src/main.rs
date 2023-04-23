use mongodb::error::Error;
use dotenvy::dotenv;

mod setup;
mod codex;

#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv().ok();

    let db_client = setup::db_connection().await?;

    let app = setup::routes(db_client).await;

    axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}