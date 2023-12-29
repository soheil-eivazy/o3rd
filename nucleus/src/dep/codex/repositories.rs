use std::str::FromStr;

use mongodb::{Client, Collection, error::Error, bson::{oid::ObjectId, doc}};
use crate::codex::models::{Entry};
use tokio_stream::StreamExt;

pub struct EntryRepo {
    collection: Collection<Entry>
}

impl EntryRepo {
    pub async fn new(client: Client) -> Self {
        EntryRepo {
            collection: client.database("o3rd").collection("entries")
        }
    }

    pub async fn insert_one(&self, title: String, body: String, publish: bool, tags: Vec<String>) -> Result<Entry, Error> {
        let entry = Entry {
            id: Some(ObjectId::new()),
            title,
            body,
            publish,
            tags,
        };

        self.collection.insert_one(entry.clone(), None).await?;
        Ok(entry)
    }

    pub async fn load_one(&self, id: String) -> Result<Option<Entry>, Error> {

        let id = ObjectId::from_str(&id).unwrap();
        let res = self.collection.find_one(doc! {"_id": id}, None).await?;
        Ok(res)
    }

    pub async fn load_many(&self) -> Result<Vec<Entry>, Error> {
        let mut cursor = self.collection.find(None , None).await?;
        let mut res = Vec::<Entry>::new();
        while let Some(doc) = cursor.next().await{
            // let e = doc?;
            res.push(doc?);
        }

        Ok(res)
    }

}


// pub struct TagRepo {
//     collection: Collection<Tag>
// }

// impl TagRepo {
//     pub async fn new(client: Client) -> Self {
//         TagRepo {
//             collection: client.database("o3rd").collection("tags")
//         }
//     }
// }