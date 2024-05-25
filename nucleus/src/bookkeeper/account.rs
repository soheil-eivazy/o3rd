use std::sync::Arc;

use axum::{extract::Path, Json, Extension};
use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use tracing::debug;
use crate::{AppState, Result};
use std::str::FromStr;

use super::model::Account;

pub async fn create(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Account> {
    // get json body
    // validate body
    // load coll 
    // 
    
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}

pub async fn load_one(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Account> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}

pub async fn load_many(Extension(state): Extension<Arc<AppState>>) -> Json<Account> {
    Json(load_test(state, "ads").await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}



pub async fn update(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Account> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}

pub async fn delete(Extension(state): Extension<Arc<AppState>>, Path(id): Path<String>) -> Json<Account> {
    Json(load_test(state, &id).await.unwrap_or_else(|ex| {
        panic!("something went wrong: {ex}")
    }))
}



async fn load_test(state: Arc<AppState>, id: &str) -> Result<Account> {
    let coll: Collection<Account> = state.db.collection("sketches");
    let cursor = coll.find_one(doc! {"_id": ObjectId::from_str(id)?}, None).await?.unwrap();
    debug!("{cursor:?}");

    Ok(cursor)
}




// what do we need here?
    // -> error handling in response
    // -> get collection from model
    // -> how to get json body
    // -> 