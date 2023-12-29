use std::sync::Arc;

use axum::{extract::Path, Json, Extension};
use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use tracing::debug;
use crate::{AppState, Result};
use std::str::FromStr;

use super::model::Sketch;


pub async fn load_one(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Sketch> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}

pub async fn load_many(Extension(state): Extension<Arc<AppState>>) -> Json<Vec<Sketch>> {
    Json(vec![load_test(state, "ads").await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    })])
}


pub async fn create(Extension(state): Extension<Arc<AppState>>) -> Json<Sketch> {
    Json(load_test(state, "&id").await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}


pub async fn update(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Sketch> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}


pub async fn delete(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Sketch> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}



async fn load_test(state: Arc<AppState>, id: &str) -> Result<Sketch> {
    let coll: Collection<Sketch> = state.db.collection("sketches");
    let cursor = coll.find_one(doc! {"_id": ObjectId::from_str(id)?}, None).await?.unwrap();
    debug!("{cursor:?}");

    Ok(cursor)
}


