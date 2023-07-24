use axum::{Router};

use log::{info};
use axum::routing::get;
use crate::controllers::root_controller::{root, root_with_path};


pub(crate) fn route_root() -> Router {
    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/ready", get(root))
        .route("/ready/:name", get(root_with_path));
    app
}




