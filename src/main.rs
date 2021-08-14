use axum::prelude::*;
use axum::AddExtensionLayer;
use std::error::Error;
use std::sync::Arc;

mod b62;
mod config;
mod kgs;
mod routes;
mod storages;

use config::{Config, StorageConfig};
use storages::{mongo::MongoStorage, Storage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match read_config() {
        Ok(config) => run_server(config).await?,
        Err(err) => eprintln!("failed to read configuration file: {}", err),
    };

    Ok(())
}

async fn run_server(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Starting node (id={})", config.node_id);

    let key_service = Arc::new(kgs::KeyGenerationService::new(config.node_id));

    let storage: Arc<dyn Storage> = match config.storage {
        StorageConfig::Mongo(config) => {
            Arc::new(MongoStorage::new(&config.uri, &config.database).await?)
        }
    };

    let app = route("/:alias", get(routes::alias_redirect))
        .route("/v0", post(routes::create_alias))
        .route("/v0/version", get(routes::version))
        .layer(AddExtensionLayer::new(key_service))
        .layer(AddExtensionLayer::new(storage));

    hyper::Server::bind(&config.listen.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn read_config() -> std::io::Result<Config> {
    let contents = std::fs::read("config.json")?;
    let config: Config = serde_json::from_slice(&contents)?;
    Ok(config)
}
