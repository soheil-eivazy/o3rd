use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genus {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain_id: ObjectId,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub amount: i64,
    pub account_id: ObjectId,
    pub genus_id: ObjectId, 
    pub created_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Month {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub start: DateTime,
    pub end: DateTime,
}