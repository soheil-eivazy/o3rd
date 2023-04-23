use axum::{
    response::IntoResponse, 
    Json, 
    extract::{Query, Path}, 
    Extension, http::StatusCode
};
use mongodb::{Client};
use crate::codex::{schemata, repositories, models};


pub async fn create_entry(Extension(db): Extension<Client>, Json(body): Json<schemata::CreateEntryInput>) -> impl IntoResponse {
    let repo = repositories::EntryRepo::new(db).await;

    match repo.insert_one(body.title, body.body, body.publish, body.tags).await {
        Ok(res) => {(
            StatusCode::CREATED, 
            Json(schemata::Output::<models::Entry>::successful(res))
        )},
        Err(err) => {(
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(schemata::Output::<models::Entry>::failed(500, err.to_string()))
        )},
    }
}

pub async fn load_entries(Extension(db): Extension<Client>, Query(_params): Query<schemata::FilterEntry>) -> impl IntoResponse {
    let repo = repositories::EntryRepo::new(db).await;

    match repo.load_many().await {
        Ok(res) => {(
                StatusCode::OK, 
                Json(schemata::Output::<Vec<models::Entry>>::successful(res))
        )},
        Err(err) => {(
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(schemata::Output::<Vec<models::Entry>>::failed(500, err.to_string()))
        )},
    }
}

pub async fn load_one_entry(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
    let repo = repositories::EntryRepo::new(db).await;

    match repo.load_one(id).await {
        Ok(res) => {
            match res {
                Some(r) => {(
                    StatusCode::OK, 
                    Json(schemata::Output::<models::Entry>::successful(r))
                )},
                _ => {(
                    StatusCode::NOT_FOUND,
                    Json(schemata::Output::<models::Entry>::failed(404, "".to_string()))
                )},
            }
        },
        Err(err) => {(
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(schemata::Output::<models::Entry>::failed(500, err.to_string()))
        )},
    }
}

pub async fn update_entry(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
    let repo = repositories::EntryRepo::new(db).await;

    match repo.load_one(id).await {
        Ok(res) => {
            match res {
                Some(r) => {(
                    StatusCode::OK, 
                    Json(schemata::Output::<models::Entry>::successful(r))
                )},
                _ => {(
                    StatusCode::NOT_FOUND,
                    Json(schemata::Output::<models::Entry>::failed(404, "".to_string()))
                )},
            }
        },
        Err(err) => {(
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(schemata::Output::<models::Entry>::failed(500, err.to_string()))
        )},
    }
}

pub async fn delete_entry(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
    let repo = repositories::EntryRepo::new(db).await;

    match repo.load_one(id).await {
        Ok(res) => {
            match res {
                Some(r) => {(
                    StatusCode::OK, 
                    Json(schemata::Output::<models::Entry>::successful(r))
                )},
                _ => {(
                    StatusCode::NOT_FOUND,
                    Json(schemata::Output::<models::Entry>::failed(404, "".to_string()))
                )},
            }
        },
        Err(err) => {(
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(schemata::Output::<models::Entry>::failed(500, err.to_string()))
        )},
    }
}





// pub async fn create_tag(Extension(db): Extension<Client>, Json(body): Json<schemata::CreateTagInput>) -> impl IntoResponse {
//     let repo = repositories::TagRepo::new(db).await;
// }

// pub async fn load_tags(Extension(db): Extension<Client>, Query(params): Query<schemata::FilterTag>) -> impl IntoResponse {
//     let repo = repositories::TagRepo::new(db).await;
// }

// pub async fn load_one_tag(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
//     let repo = repositories::TagRepo::new(db).await;
// }

// pub async fn update_tag(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
//     let repo = repositories::TagRepo::new(db).await;
// }

// pub async fn delete_tag(Extension(db): Extension<Client>, Path(id): Path<String>) -> impl IntoResponse {
//     let repo = repositories::TagRepo::new(db).await;
// }