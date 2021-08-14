use async_trait::async_trait;
use std::error::Error;

pub mod mongo;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get(&self, alias: &String) -> Result<Option<String>, Box<dyn Error>>;

    async fn put(&self, alias: &String, url: &String) -> Result<(), Box<dyn Error>>;
}
