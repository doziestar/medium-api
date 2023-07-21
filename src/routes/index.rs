use axum::{
    routing::get,
    Router,
};
use crate::routes::root::root;

pub(crate) async fn get_routes() -> Router<()> {
    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/ready", get(root));
    app
}
