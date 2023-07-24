use axum::{Json};
use serde_json::json;

use log::{info};
use serde_json;
use std::error::Error;
use axum::extract::{Path, Query};
use crate::dto::root_dto::Message;


pub async fn root(params: Query<Message>) -> Json<serde_json::Value> {
    info!("I am ready to serve you");
    let name = params.message.as_deref().unwrap_or("stranger");
    let message = format!("I am ready to serve you {}", name);
    Json(json!({
        "message": message,
    }))
}

pub async fn root_with_path(Path(name): Path<String>) -> Json<serde_json::Value> {
    info!("I am ready to serve you");
    let message = format!("I am ready to serve you {}", name);
    Json(json!({
        "message": message,
    }))
}
