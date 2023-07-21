use axum::Json;
use serde_json::json;
use tracing::info;

pub(crate) async fn root() -> Json<serde_json::Value> {
    info!("I am ready to serve you");
    Json(json!({
        "message": "I am ready to serve you",
    }))
}