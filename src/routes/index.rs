use axum::{
    routing::get,
    Router,
};
use crate::routes::root::route_root;

pub(crate) async fn get_routes() -> Router<()> {
    let app = Router::new().nest("/api", route_root());
    app

    // let app = Router::new().merge(route_root());
    // app
}
