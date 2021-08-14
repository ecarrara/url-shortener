use super::b62;
use super::kgs;
use super::storages;
use axum::extract::{Extension, Json};
use axum::http::{header, Response, StatusCode};
use axum::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Version {
    pub version: String,
}

pub async fn version() -> response::Json<Version> {
    response::Json(Version {
        version: "0.0.1".to_string(),
    })
}

#[derive(Deserialize, Debug)]
pub struct CreateAliasRequest {
    pub long_url: String,
}

#[derive(Serialize, Debug)]
pub struct CreateAliasResponse {
    pub long_url: String,
    pub alias: String,
}

pub struct AppError {}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(_inner: Box<dyn std::error::Error>) -> Self {
        Self {}
    }
}

impl response::IntoResponse for AppError {
    fn into_response(self) -> axum::http::Response<Body> {
        let mut response =
            response::Json(json!({"message": "Internal server error."})).into_response();
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        response
    }
}

pub async fn create_alias(
    Extension(key_service): Extension<Arc<kgs::KeyGenerationService>>,
    Extension(storage): Extension<Arc<dyn storages::Storage>>,
    Json(payload): extract::Json<CreateAliasRequest>,
) -> Result<response::Json<CreateAliasResponse>, AppError> {
    let key = key_service.get();
    let alias = b62::encode(key);

    storage.put(&alias, &payload.long_url).await?;

    Ok(response::Json(CreateAliasResponse {
        alias,
        long_url: payload.long_url,
    }))
}

pub async fn alias_redirect(
    Extension(storage): Extension<Arc<dyn storages::Storage>>,
    extract::Path(alias): extract::Path<String>,
) -> Result<Response<Body>, AppError> {
    match storage.get(&alias).await? {
        Some(url) => Ok(Response::builder()
            .header(header::LOCATION, url)
            .status(StatusCode::TEMPORARY_REDIRECT)
            .body(Body::empty())
            .unwrap()),
        None => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}
