mod routes;

use axum::{
    routing::get,
    Router,
};

use tracing_subscriber;
use tracing::{info, span, Level};
use crate::routes::index::get_routes;

#[tokio::main]
async fn main() {
    // set up tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        // .json()
        .init();
    // build our application with a single route
    let app = get_routes().await;

    info!("starting server on localhost:3000");


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}