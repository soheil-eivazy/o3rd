use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {    
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub body: String,
    pub publish: bool,
    pub tags: Vec<String>,
}


#[derive(Serialize, Deserialize)]
pub struct Tag {    
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    title: String,
}