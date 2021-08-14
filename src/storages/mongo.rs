use super::Storage;
use mongodb::{
    bson::{doc, Document},
    Client,
};
use std::error::Error;

pub struct MongoStorage {
    client: Client,
    database: String,
}

impl MongoStorage {
    pub async fn new(uri: &str, database: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Client::with_uri_str(uri).await?,
            database: database.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Storage for MongoStorage {
    async fn get(&self, alias: &String) -> Result<Option<String>, Box<dyn Error>> {
        let collection = self
            .client
            .database(&self.database)
            .collection::<Document>("urls");

        match collection.find_one(doc! { "_id": alias }, None).await {
            Ok(Some(doc)) => Ok(doc.get("url").unwrap().as_str().map(|s| s.to_string())),
            Ok(None) => Ok(None),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn put(&self, alias: &String, url: &String) -> Result<(), Box<dyn Error>> {
        let collection = self.client.database(&self.database).collection("urls");
        collection
            .insert_one(
                doc! {
                    "_id": alias,
                    "url": url
                },
                None,
            )
            .await?;
        Ok(())
    }
}
